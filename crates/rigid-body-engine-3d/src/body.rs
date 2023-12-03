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
    pub mass: f32,
    pub locked: bool,
}

impl Default for RigidBodyProperties {
    fn default() -> Self {
        Self {
            mass: 1.0,
            locked: false,
        }
    }
}


#[derive(Component, Clone, Default, Reflect, Debug)]
#[reflect(Debug, Default)]
pub struct RigidBodyState {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub old_acceleration: Vec3,
}