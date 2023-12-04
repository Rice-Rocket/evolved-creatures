use bevy::prelude::*;

use crate::prelude::{RigidBodyState, RigidBodyProperties, RigidBodyImpulseAccumulator, RigidBodySimulationSettings};


#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone)]
pub struct ApplyForcesSet;


pub(crate) fn apply_gravity(
    mut bodies: Query<(&mut RigidBodyState, &RigidBodyProperties)>,
) {
    for (mut state, props) in bodies.iter_mut() {
        state.force += Vec3::new(0.0, -10.0, 0.0) * props.mass;
    }
}

pub(crate) fn apply_collisions(
    mut bodies: Query<(&mut RigidBodyState, &RigidBodyProperties)>,
) {
    let mut combinations = bodies.iter_combinations_mut();
    while let Some([(mut state_1, props_1), (mut state_2, props_2)]) = combinations.fetch_next() {
        apply_single_collision(state_1.as_mut(), props_1, state_2.as_mut(), props_2);
        apply_single_collision(state_2.as_mut(), props_2, state_1.as_mut(), props_1);
    }
}

fn apply_single_collision(
    state_1: &mut RigidBodyState,
    props_1: &RigidBodyProperties,
    state_2: &mut RigidBodyState,
    props_2: &RigidBodyProperties,
) {
    for vertex in props_1.vertices.as_ref().unwrap().iter() {
        let p = state_1.globalize(*vertex);
        if state_2.intersects(p, props_2.scale) {

            let exterior_point = state_2.exterior_point(p, props_2.scale);
            let d = exterior_point - p;
            let dist = d.length();

            if dist == 0.0 { continue };

            let kt = props_2.roughness * 4.0;
            let kn = (1.0 - props_2.resilience).powi(10);
            let kc = props_2.hardness * 1000.0;

            let v = state_1.velocity_at_point(p);
            let vvel = state_2.velocity_at_point(p);
            let vdiff = v - vvel;
            let n = d / dist;
            let vn = vdiff.dot(n) * n;
            let vt = (vn * n - vdiff) * kt;
            let b = (kc * props_1.mass).sqrt() * 2.0 * kn;
            let f = -(d * kc - b * vn + vt) * props_1.mass;

            if !props_1.locked {
                let center = state_1.position;
                state_1.force -= f;
                state_1.torque += (p - center).cross(-f);
            }

            if !props_2.locked {
                let center = state_2.position;
                state_2.force += f;
                state_2.torque += (p - center).cross(f);
            }
        }
    }
}

pub(crate) fn apply_accumulated_impulses(
    mut bodies: Query<(&mut RigidBodyState, &mut RigidBodyImpulseAccumulator)>,
    settings: Res<RigidBodySimulationSettings>,
) {
    for (mut state, mut impulses) in bodies.iter_mut() {
        state.force += impulses.force / settings.sub_dt;
        state.torque += impulses.torque / settings.sub_dt;

        impulses.force = Vec3::ZERO;
        impulses.torque = Vec3::ZERO;
    }
}