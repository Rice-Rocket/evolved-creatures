use bevy::prelude::*;

use super::{RBJointType, RBJointProperties};

#[derive(Component, Debug, Reflect)]
#[reflect(Debug, Default)]
pub struct RBRigidJoint {
    pub connection_separation: f32,
}

impl Default for RBRigidJoint {
    fn default() -> Self {
        Self {
            connection_separation: 1.0,
        }
    }
}

impl RBJointType for RBRigidJoint {
    fn connection_points(&self, props: &RBJointProperties) -> Vec<(Vec3, Vec3)> {
        let offsets = [
            (props.tangent * self.connection_separation + props.bitangent * self.connection_separation),
            (-props.tangent * self.connection_separation + props.bitangent * self.connection_separation),
            (props.tangent * self.connection_separation + -props.bitangent * self.connection_separation),
            (-props.tangent * self.connection_separation + -props.bitangent * self.connection_separation),
        ];
        vec![
            (props.position_1 + offsets[0], props.position_2 + offsets[0]),
            (props.position_1 + offsets[1], props.position_2 + offsets[1]),
            (props.position_1 + offsets[2], props.position_2 + offsets[2]),
            (props.position_1 + offsets[3], props.position_2 + offsets[3]),
        ]
    }
    fn locked_limits(&self) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
    }
}

