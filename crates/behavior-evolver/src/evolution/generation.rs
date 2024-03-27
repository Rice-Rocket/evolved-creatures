use bevy::prelude::*;
use bevy_rapier3d::dynamics::Velocity;
use creature_builder::{builder::node::CreatureMorphologyGraph, limb::CreatureLimb};

use super::{
    fitness::{EvolutionFitnessEval, FitnessEvalInput},
    state::EvolutionState,
};

#[derive(Resource, Default)]
pub struct EvolutionGeneration<F: EvolutionFitnessEval + Send + Sync + Default + 'static> {
    pub(crate) population: Vec<CreatureMorphologyGraph>,
    pub(crate) fitnesses: Vec<f32>,
    pub(crate) current_test: Option<usize>,
    pub(crate) current_fitness: Option<F>,
}


pub(crate) fn test_generation<F: EvolutionFitnessEval + Send + Sync + Default + 'static>(
    mut generation: ResMut<EvolutionGeneration<F>>,
    state: Res<State<EvolutionState>>,
    mut next_state: ResMut<NextState<EvolutionState>>,
    limbs: Query<(&CreatureLimb, &Transform, &Velocity)>,
) {
    match state.get() {
        EvolutionState::EvaluatingCreature => {
            match generation.current_test {
                Some(i) => {
                    let eval = generation.current_fitness.as_ref().unwrap().final_eval();
                    generation.fitnesses.push(eval);
                    generation.current_test = Some(i + 1)
                },
                None => {
                    generation.fitnesses.clear();
                    generation.current_fitness = Some(F::default());
                    generation.current_test = Some(0)
                },
            };
            if generation.current_test.unwrap() < generation.population.len() {
                // Still testing generation
                next_state.set(EvolutionState::TestingCreature)
            } else {
                // Finished testing generation
                generation.current_test = None;
                generation.current_fitness = None;
                next_state.set(EvolutionState::PopulatingGeneration)
            }
        },
        EvolutionState::TestingCreature => {
            let index = generation.current_test.unwrap();
            let morph = &generation.population[index];
            let creature_id = morph.creature;
            let (mut position, mut velocity) = (Vec3::ZERO, Vec3::ZERO);

            limbs.iter().filter(|(limb, _, _)| limb.creature == creature_id).for_each(|(_, pos, vel)| {
                position += pos.translation;
                velocity += vel.linvel;
            });

            generation.current_fitness.as_mut().unwrap().eval_continuous(FitnessEvalInput { position, velocity });
        },

        _ => (),
    }
}
