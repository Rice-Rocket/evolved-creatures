use bevy::prelude::*;



#[derive(Bundle, Clone, Default)]
pub struct RigidBodyObject {
    pub state: RigidBodyState,
    pub properties: RigidBodyProperties,
    pub object: PbrBundle,
}


#[derive(Component, Clone, Reflect, Debug)]
#[reflect(Debug, Default)]
pub struct RigidBodyProperties {
    pub scale: Vec3,
    pub mass: f32,
    pub hardness: f32,
    pub resilience: f32,
    pub roughness: f32,
    pub moments: Option<Vec3>,
    pub locked: bool,
}

impl Default for RigidBodyProperties {
    fn default() -> Self {
        Self {
            scale: Vec3::ONE,
            hardness: 1.0,
            resilience: 0.2,
            roughness: 1.0,
            mass: 1.0,
            moments: None,
            locked: false,
        }
    }
}

pub(crate) fn initialize_bodies(
    mut bodies: Query<&mut RigidBodyProperties>
) {
    for mut props in bodies.iter_mut() {
        props.moments = Some(props.scale.normalize());
    }
}


#[derive(Component, Clone, Default, Reflect, Debug)]
#[reflect(Debug, Default)]
pub struct RigidBodyState {
    pub position: Vec3,
    pub orientation: Quat,

    pub velocity: Vec3,
    pub angular_velocity: Vec3,
    pub angular_momentum: Vec3,

    pub acceleration: Vec3,
    pub force: Vec3,
    pub torque: Vec3,
}

impl RigidBodyState {
    pub fn localize(&self, point: Vec3) -> Vec3 {
        self.orientation.inverse() * (point - self.position)
    }

    pub fn sdf(&self, point: Vec3, scale: Vec3) -> f32 {
        let p = self.localize(point);
        let q = p.abs() - scale / 2.0;
        q.max(Vec3::ZERO).length() + q.max_element().min(0.0)
    }

    pub fn sdf_gradient(&self, point: Vec3, scale: Vec3) -> Vec3 {
        let p = self.localize(point);
        let w = p.abs() - scale / 2.0;
        let s = Vec3::new(
            if p.x < 0.0 { -1.0 } else { 1.0 },
            if p.y < 0.0 { -1.0 } else { 1.0 },
            if p.z < 0.0 { -1.0 } else { 1.0 },
        );

        let g = w.max_element();
        let q = w.max(Vec3::ZERO);
        let l = q.length();

        return s * (
            if g > 0.0 { q / l }
            else {
                if w.x > w.y && w.x > w.z {
                    Vec3::new(1.0, 0.0, 0.0)
                } else {
                    if w.y > w.z {
                        Vec3::new(0.0, 1.0, 0.0)
                    } else {
                        Vec3::new(0.0, 0.0, 1.0)
                    }
                }
            }
        );
    }

    pub fn exterior_point(&self, point: Vec3, scale: Vec3) -> Vec3 {
        point + self.sdf_gradient(point, scale) * -self.sdf(point, scale)
    }

    pub fn intersects(&self, point: Vec3, scale: Vec3) -> bool {
        self.sdf(point, scale) <= 0.0
    }

    pub fn velocity_at_point(&self, point: Vec3) -> Vec3 {
        let arm = point - self.position;
        self.angular_velocity.cross(arm) + self.velocity
    }

    pub fn vertices(&self, scale: Vec3) -> Vec<Vec3> {
        let h = scale / 2.0;
        [
            Vec3::new(-h.x, -h.y, -h.z),
            Vec3::new(-h.x, -h.y, h.z),
            Vec3::new(-h.x, h.y, -h.z),
            Vec3::new(h.x, -h.y, -h.z),
            Vec3::new(-h.x, h.y, h.z),
            Vec3::new(h.x, h.y, -h.z),
            Vec3::new(h.x, -h.y, h.z),
            Vec3::new(h.x, h.y, h.z),
        ].iter().map(|x| {
            (self.orientation * *x) + self.position
        }).collect()
    }

    pub fn axes(&self) -> [Vec3; 3] {
        [
            /* self.rotation * */ Vec3::X,
            Vec3::Y,
            Vec3::Z,
        ]
    }
}