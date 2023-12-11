use bevy::prelude::*;

use super::{RBJointType, RBJointProperties};

/// AKA Hinge joint
#[derive(Component, Debug, Reflect)]
#[reflect(Debug, Default)]
pub struct RBRevoluteJoint {
    pub connection_separation: f32,
}

impl Default for RBRevoluteJoint {
    fn default() -> Self {
        Self {
            connection_separation: 1.0,
        }
    }
}

impl RBJointType for RBRevoluteJoint {
    fn connection_points(&self, props: &RBJointProperties) -> Vec<(Vec3, Vec3)> {
        let offsets = [
            props.tangent * self.connection_separation,
            -props.tangent * self.connection_separation,
        ];
        vec![
            (props.position_1 + offsets[0], props.position_2 + offsets[0]),
            (props.position_1 + offsets[1], props.position_2 + offsets[1]),
        ]
    }
}

