use bevy::prelude::*;
use bevy_rapier3d::geometry::{Group, CollisionGroups};


#[derive(Resource)]
pub struct CreatureBuilderGlobals {
}

impl Default for CreatureBuilderGlobals {
    fn default() -> Self {
        Self {
        }
    }
}

impl CreatureBuilderGlobals {
    pub const MAIN_COLLISION_GROUP: CollisionGroups = CollisionGroups::new(Group::GROUP_1, Group::GROUP_2);
    pub const GROUND_COLLISION_GROUP: CollisionGroups = CollisionGroups::new(Group::GROUP_2, Group::GROUP_1);
}