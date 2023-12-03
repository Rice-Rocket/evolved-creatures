use bevy::prelude::*;

pub mod prelude;
pub mod body;

use body::*;

pub struct RigidBodySimulationPlugin;

impl Plugin for RigidBodySimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<RigidBodyProperties>()
            .register_type::<RigidBodyState>();
    }
}