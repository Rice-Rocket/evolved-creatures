use bevy::{prelude::*, utils::HashSet};

use crate::prelude::{RigidBodyState, RigidBodyProperties};


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
    mut gizmos: Gizmos,
    mut bodies: Query<(Entity, &mut RigidBodyState, &RigidBodyProperties)>,
) {
    let mut combinations = bodies.iter_combinations_mut();
    let mut collided_pairs = HashSet::new();
    while let Some([(entity_1, mut state_1, props_1), (entity_2, mut state_2, props_2)]) = combinations.fetch_next() {
        if collided_pairs.contains(&(entity_2, entity_1)) { continue };

        for vertex in state_1.vertices(props_1.scale).iter() {
            if state_2.intersects(*vertex, props_2.scale) {
                collided_pairs.insert((entity_1, entity_2));

                let exterior_point = state_2.exterior_point(*vertex, props_2.scale);
                let d = exterior_point - *vertex;
                let dist = d.length();

                if dist == 0.0 { continue };

                let kt = props_2.roughness * 4.0;
                let kn = (1.0 - props_2.resilience).powi(10);
                let kc = props_2.hardness * 1000.0;

                let v = state_1.velocity_at_point(*vertex);
                let vvel = state_2.velocity_at_point(*vertex);
                let vdiff = v - vvel;
                let n = d / dist;
                let vn = vdiff.dot(n) * n;
                let vt = (vn * n - vdiff) * kt;
                let b = (kc * props_1.mass).sqrt() * 2.0 * kn;
                let f = -(d * kc - b * vn + vt) * props_1.mass;

                if !props_1.locked {
                    let center = state_1.position;
                    state_1.force -= f;
                    state_1.torque += (*vertex - center).cross(-f);
                }

                if !props_2.locked {
                    let center = state_2.position;
                    state_2.force += f;
                    state_2.torque += (*vertex - center).cross(f);
                }
            }
        }
    }
}