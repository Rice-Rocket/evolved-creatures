pub mod limb;
pub mod joint;
pub mod sensor;
pub mod config;
pub mod builder;
pub mod effector;
pub mod expr;


use bevy::prelude::*;
use config::CreatureBuilderConfig;
use sensor::update_sensor_status;


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct CreatureId(pub usize);



pub struct CreatureBuilderPlugin;


impl Plugin for CreatureBuilderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CreatureBuilderConfig>()
            .add_systems(Update, update_sensor_status);
    }
}