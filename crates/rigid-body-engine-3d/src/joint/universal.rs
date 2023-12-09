use bevy::prelude::*;

use super::{RBJointType, RBJointProperties};

#[derive(Component, Debug, Reflect)]
#[reflect(Debug, Default)]
pub struct RBUniversalJoint {
}

impl Default for RBUniversalJoint {
    fn default() -> Self {
        Self {
        }
    }
}

impl RBJointType for RBUniversalJoint {
    fn connection_points(&self, props: &RBJointProperties) -> Vec<(Vec3, Vec3)> {
        vec![(props.position_1, props.position_2)]
    }
}

