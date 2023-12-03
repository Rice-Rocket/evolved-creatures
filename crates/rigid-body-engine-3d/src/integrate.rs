use bevy::prelude::*;

use crate::prelude::{RigidBodyState, RigidBodySimulationSettings, RigidBodyProperties};


pub(crate) fn update_positions(
    mut bodies: Query<(&mut RigidBodyState, &RigidBodyProperties)>,
    settings: Res<RigidBodySimulationSettings>,
) {
    let dt = settings.sub_dt;
    for (mut state, props) in bodies.iter_mut() {
        if props.locked { continue };
        state.position = state.position + state.velocity * dt + 0.5 * state.old_acceleration / props.mass * dt * dt;
    }
}

pub(crate) fn update_velocities(
    mut bodies: Query<(&mut RigidBodyState, &RigidBodyProperties)>,
    settings: Res<RigidBodySimulationSettings>,
) {
    let dt = settings.sub_dt;
    for (mut state, props) in bodies.iter_mut() {
        if props.locked {
            state.velocity = Vec3::ZERO;
            state.acceleration = Vec3::ZERO;
            state.old_acceleration = Vec3::ZERO;
            continue;
        }
        
        state.velocity = state.velocity + 0.5 * (state.old_acceleration + state.acceleration) / props.mass * dt;
        state.old_acceleration = state.acceleration;
        state.acceleration = Vec3::ZERO;
    }
}

pub(crate) fn update_object_transform(
    mut bodies: Query<(&mut Transform, &RigidBodyState)>,
) {
    for (mut transform, state) in bodies.iter_mut() {
        transform.translation = state.position;
    }
}