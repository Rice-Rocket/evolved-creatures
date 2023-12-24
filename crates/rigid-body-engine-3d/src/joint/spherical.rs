use bevy::prelude::*;

use super::{RBJointType, RBJointProperties};

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Debug, Default)]
pub struct RBSphericalJoint;


impl RBJointType for RBSphericalJoint {
    fn connection_points(&self, props: &RBJointProperties) -> Vec<(Vec3, Vec3)> {
        vec![(props.position_1, props.position_2)]
    }
    fn locked_limits(&self) -> Vec2 {
        Vec2::new(1.0, 1.0)
    }
}

