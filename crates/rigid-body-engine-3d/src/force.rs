use bevy::prelude::*;

use crate::prelude::{RigidBodyState, RigidBodyProperties};


#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone)]
pub struct ApplyForcesSet;


pub(crate) fn apply_gravity(
    mut bodies: Query<(&mut RigidBodyState, &RigidBodyProperties)>,
) {
    for (mut state, props) in bodies.iter_mut() {
        state.acceleration += Vec3::new(0.0, -100.0, 0.0) * props.mass;
    }
}