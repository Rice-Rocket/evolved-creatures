use bevy_rapier3d::prelude::*;
use bevy::prelude::*;


#[derive(Component)]
pub struct CreatureLimb;



#[derive(Bundle)]
pub struct CreatureLimbBundle {
    pub limb: CreatureLimb,

    // Rigid body
    pub rb: RigidBody,
    pub velocity: Velocity,
    pub gravity: GravityScale,
    pub forces: ExternalForce,

    // Collider
    pub collider: Collider,
    pub collision_groups: CollisionGroups,
    // pub sensor: Sensor,
    pub friction: Friction,
    pub restitution: Restitution,
    pub mass: ColliderMassProperties,

    // Material and Mesh
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,

    // Transform
    pub transform: Transform,
    pub(crate) global_transform: GlobalTransform,

    // Visibility
    pub(crate) visibility: Visibility,
    pub(crate) inherited_visibility: InheritedVisibility,
    pub(crate) view_visibility: ViewVisibility,

    #[bundle(ignore)]
    color: Color,
}




impl Default for CreatureLimbBundle {
    fn default() -> Self {
        CreatureLimbBundle {
            limb: CreatureLimb,

            rb: RigidBody::Dynamic,
            velocity: Velocity { linvel: Vec3::ZERO, angvel: Vec3::ZERO },
            gravity: GravityScale(1.0),
            forces: ExternalForce { force: Vec3::ZERO, torque: Vec3::ZERO },

            collider: Collider::cuboid(1.0, 1.0, 1.0),
            collision_groups: CollisionGroups::default(),
            // sensor: Sensor,
            friction: Friction { coefficient: 0.5, combine_rule: CoefficientCombineRule::Average },
            restitution: Restitution { coefficient: 0.1, combine_rule: CoefficientCombineRule::Average },
            mass: ColliderMassProperties::Mass(1.0),

            mesh: Handle::default(),
            material: Handle::default(),

            transform: Transform::from_translation(Vec3::ZERO),
            global_transform: GlobalTransform::default(),

            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),

            color: Color::WHITE,
        }
    }
}




impl CreatureLimbBundle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_size(mut self, half_size: Vec3) -> Self {
        self.collider.set_scale(half_size, 0);
        self.transform.scale = half_size;
        self
    }
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    pub fn with_groups(mut self, groups: CollisionGroups) -> Self {
        self.collision_groups = groups;
        self
    }
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }
    pub fn with_initial_force(mut self, force: Vec3) -> Self {
        self.forces.force = force;
        self
    }
    pub fn with_initial_torque(mut self, torque: Vec3) -> Self {
        self.forces.torque = torque;
        self
    }
    pub fn finish(mut self, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>) -> Self {
        self.mesh = meshes.add(Mesh::from(shape::Box::new(2.0, 2.0, 2.0)));
        self.material = materials.add(StandardMaterial::from(self.color));
        self
    }
}

