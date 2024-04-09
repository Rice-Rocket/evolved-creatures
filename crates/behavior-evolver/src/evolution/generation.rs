use std::time::{Duration, Instant};

use bevy::prelude::*;
use bevy_rapier3d::dynamics::Velocity;
use creature_builder::{builder::node::CreatureMorphologyGraph, limb::CreatureLimb, CreatureId};

use super::{
    fitness::{EvolutionFitnessEval, FitnessEvalInput},
    state::EvolutionState,
};


#[derive(Resource)]
pub struct GenerationTestingConfig {
    pub test_time: Duration,
}

impl Default for GenerationTestingConfig {
    fn default() -> Self {
        Self { test_time: Duration::from_secs_f32(3.0) }
    }
}


#[derive(Resource, Default)]
pub struct EvolutionGeneration<F: EvolutionFitnessEval + Send + Sync + Default + 'static> {
    pub(crate) population: Vec<CreatureMorphologyGraph>,
    pub(crate) fitnesses: Vec<f32>,
    pub(crate) current_test: Option<usize>,
    pub(crate) current_fitness: Option<F>,
    pub(crate) current_start_time: Option<Instant>,
    pub(crate) current_creature: Option<CreatureId>,
}


pub(crate) fn test_generation<F: EvolutionFitnessEval + Send + Sync + Default + 'static>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut generation: ResMut<EvolutionGeneration<F>>,
    config: Res<GenerationTestingConfig>,
    state: Res<State<EvolutionState>>,
    mut next_state: ResMut<NextState<EvolutionState>>,
    limbs: Query<(Entity, &CreatureLimb, &Transform, &Velocity)>,
) {
    match state.get() {
        EvolutionState::EvaluatingCreature => {
            info!("evaluating creature");
            match generation.current_test {
                Some(i) => {
                    let eval = generation.current_fitness.as_ref().unwrap().final_eval();
                    generation.fitnesses.push(eval);
                    generation.current_fitness = Some(F::default());
                    generation.current_test = Some(i + 1);
                },
                None => {
                    generation.fitnesses.clear();
                    generation.current_fitness = Some(F::default());
                    generation.current_test = Some(0);
                },
            };
            if generation.current_test.unwrap() < generation.population.len() {
                // Still testing generation
                generation.current_start_time = Some(Instant::now());
                if let Some(id) = generation.current_creature {
                    limbs
                        .iter()
                        .filter(|(_, limb, _, _)| limb.creature == id)
                        .for_each(|(entity, _, _, _)| commands.entity(entity).despawn());
                }
                let morph = &generation.population[generation.current_test.unwrap()];
                let mut result = morph.evaluate();
                generation.current_creature = Some(morph.creature);
                result.align_to_ground();
                result.build(&mut commands, &mut meshes, &mut materials);
                next_state.set(EvolutionState::TestingCreature);
            } else {
                // Finished testing generation
                generation.current_test = None;
                generation.current_fitness = None;
                next_state.set(EvolutionState::PopulatingGeneration);
            }
        },
        EvolutionState::TestingCreature => {
            let index = generation.current_test.unwrap();
            let morph = &generation.population[index];
            let creature_id = morph.creature;

            let limb_pos_vels: Vec<_> =
                limbs.iter().filter(|(_, limb, _, _)| limb.creature == creature_id).map(|(_, _, pos, vel)| (*pos, *vel)).collect();

            generation.current_fitness.as_mut().unwrap().eval_continuous(FitnessEvalInput { limbs: limb_pos_vels });

            if generation.current_start_time.unwrap().elapsed() > config.test_time {
                // Finished testing creature
                next_state.set(EvolutionState::EvaluatingCreature);
            }
        },

        _ => (),
    }
}
