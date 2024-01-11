pub mod limb;
pub mod globals;

use bevy::prelude::*;
use globals::CreatureBuilderGlobals;



pub struct CreatureBuilderPlugin;


impl Plugin for CreatureBuilderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CreatureBuilderGlobals>();
    }
}