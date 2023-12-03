use bevy::{prelude::*, utils::HashSet};

use crate::prelude::{RigidBodyState, RigidBodyProperties};


#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone)]
pub struct ApplyForcesSet;


pub(crate) fn apply_gravity(
    mut bodies: Query<(&mut RigidBodyState, &RigidBodyProperties)>,
) {
    for (mut state, props) in bodies.iter_mut() {
        state.force += Vec3::new(0.0, -1.0, 0.0) * props.mass;
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

                gizmos.line(*vertex, state_2.exterior_point(*vertex, props_2.scale), Color::BLUE);
                gizmos.sphere(Vec3::ZERO, Quat::IDENTITY, 5.0, Color::GREEN);
            }
        }
    }
}