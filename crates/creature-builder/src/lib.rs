pub mod limb;
pub mod joint;
pub mod sensor;
pub mod config;
pub mod graph;


use bevy::prelude::*;
use config::CreatureBuilderConfig;
use sensor::update_sensor_status;



pub struct CreatureBuilderPlugin;


impl Plugin for CreatureBuilderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CreatureBuilderConfig>()
            .add_systems(Update, update_sensor_status);
    }
}