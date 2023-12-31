use bevy::prelude::*;



#[derive(Bundle, Clone, Default)]
pub struct RigidBodyObject {
    pub state: RigidBodyState,
    pub properties: RigidBodyProperties,
    pub impulses: RigidBodyImpulseAccumulator,
    pub object: PbrBundle,
}


#[derive(Component, Clone, Reflect, Debug, Default)]
#[reflect(Debug, Default)]
pub struct RigidBodyImpulseAccumulator {
    pub force: Vec3,
    pub torque: Vec3,
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
    pub collision_point_density: UVec3,
    pub locked: bool,
    pub is_collider: bool,
    pub vertices: Option<Vec<Vec3>>,
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
            collision_point_density: UVec3::new(2, 2, 2),
            vertices: None,
            locked: false,
            is_collider: true,
        }
    }
}

impl RigidBodyProperties {
    pub(crate) fn inverse_moment_mat(&self, orientation: Mat3) -> Mat3 {
        let mut rt = orientation.transpose();
        let moments = self.moments.unwrap() * self.mass;

        rt.x_axis /= moments.x;
        rt.y_axis /= moments.y;
        rt.z_axis /= moments.z;

        orientation * rt
    }
    pub(crate) fn inverse_moment_quat(&self, orientation: Quat) -> Mat3 {
        let r = Mat3::from_quat(orientation);
        self.inverse_moment_mat(r)
    }
}

pub(crate) fn initialize_bodies(
    mut bodies: Query<&mut RigidBodyProperties>
) {
    for mut props in bodies.iter_mut() {
        props.moments = Some(props.scale.normalize());

        let mut vertices = Vec::new();
        for (local_up, local_x, local_y) in [
            (Vec3::Y, 0, 2), (Vec3::NEG_Y, 0, 2), (Vec3::X, 2, 1), 
            (Vec3::NEG_X, 2, 1), (Vec3::Z, 1, 0), (Vec3::NEG_Z, 1, 0)
        ] {
            let axis_a = Vec3::new(local_up.y, local_up.z, local_up.x);
            let axis_b = local_up.cross(axis_a);

            for x in 0..props.collision_point_density[local_x] {
                for y in 0..props.collision_point_density[local_y] {
                    let uv = Vec2::new(x as f32, y as f32) / Vec2::new(props.collision_point_density[local_x] as f32 - 1.0, props.collision_point_density[local_y] as f32 - 1.0);
                    let p = local_up * 0.5 + (uv.x - 0.5) * axis_a + (uv.y - 0.5) * axis_b;

                    if !vertices.contains(&p) {
                        vertices.push(p);
                    }
                }
            }
        }
        vertices = vertices.iter().map(|x| *x * props.scale).collect();
        props.vertices = Some(vertices);
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
    pub fn globalize(&self, point: Vec3) -> Vec3 {
        (self.orientation * point) + self.position
    }

    pub fn localize_bivec(&self, dir: Vec3) -> Vec3 {
        self.orientation.inverse() * dir
    }
    pub fn globalize_bivec(&self, dir: Vec3) -> Vec3 {
        self.orientation * dir
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

        return self.orientation * (s * (
            if g > 0.0 { q / l }
            else {
                if w.x > w.y && w.x > w.z {
                    Vec3::X
                } else {
                    if w.y > w.z {
                        Vec3::Y
                    } else {
                        Vec3::Z
                    }
                }
            }
        ));
    }

    pub fn exterior_point(&self, point: Vec3, scale: Vec3) -> Vec3 {
        point - self.sdf_gradient(point, scale) * self.sdf(point, scale)
    }

    pub fn intersects(&self, point: Vec3, scale: Vec3) -> bool {
        self.sdf(point, scale) <= 0.0
    }

    pub fn velocity_at_point(&self, point: Vec3) -> Vec3 {
        let arm = point - self.position;
        self.angular_velocity.cross(arm) + self.velocity
    }

    pub fn apply_force(&mut self, point: Vec3, f: Vec3) {
        let arm = point - self.position;
        self.force += f;
        self.torque += arm.cross(f);
    }
}