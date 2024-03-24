use bevy::prelude::*;

use crate::prelude::{RigidBodyProperties, RigidBodySimulationSettings, RigidBodyState};


pub(crate) fn update_positions(mut bodies: Query<(&mut RigidBodyState, &RigidBodyProperties)>, settings: Res<RigidBodySimulationSettings>) {
    let dt = settings.sub_dt;
    for (mut state, props) in bodies.iter_mut() {
        if props.locked {
            continue;
        };
        let vh = 0.5 * state.acceleration * dt + state.velocity;
        state.position += vh * dt;
        state.velocity = vh;


        let lh = 0.5 * state.torque * dt + state.angular_momentum;
        let r = Mat3::from_quat(state.orientation);

        let inverse_moment = props.inverse_moment_mat(r);
        let ang_vel = inverse_moment * lh;
        let rdot = Mat3::from_cols(
            Vec3::new(0.0, ang_vel.z, -ang_vel.y),
            Vec3::new(-ang_vel.z, 0.0, ang_vel.x),
            Vec3::new(ang_vel.y, -ang_vel.x, 0.0),
        ) * r;

        state.orientation = Quat::from_mat3(&(r + rdot * dt)).normalize();
        state.angular_momentum = lh;
        state.angular_velocity = ang_vel;
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
            state.force = Vec3::ZERO;
            state.torque = Vec3::ZERO;
            continue;
        }

        state.acceleration = state.force / props.mass;
        state.velocity = 0.5 * state.acceleration * dt + state.velocity;

        state.angular_momentum = 0.5 * state.torque * dt + state.angular_momentum;

        state.force = Vec3::ZERO;
        state.torque = Vec3::ZERO;
    }
}

pub(crate) fn update_object_transform(mut bodies: Query<(&mut Transform, &RigidBodyState, &RigidBodyProperties)>) {
    for (mut transform, state, props) in bodies.iter_mut() {
        transform.translation = state.position;
        transform.scale = props.scale;
        transform.rotation = state.orientation;
    }
}
