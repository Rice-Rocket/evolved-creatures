use bevy::prelude::*;

use super::{RBJointType, RBJointProperties};

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Debug, Default)]
pub struct RBSphericalJoint;


impl RBJointType for RBSphericalJoint {
    fn connection_points(&self, props: &RBJointProperties) -> Vec<(Vec3, Vec3)> {
        vec![(props.position_1, props.position_2)]
    }
}
