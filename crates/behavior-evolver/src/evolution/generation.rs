use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier3d::{
    dynamics::Velocity,
    geometry::{Friction, Restitution},
};
use creature_builder::{builder::node::CreatureMorphologyGraph, config::CreatureBuilderConfig, limb::CreatureLimb, CreatureId};

use super::{
    fitness::{EvolutionFitnessEval, FitnessEvalInput},
    populate::CreaturePopulateFlag,
    state::{EvolutionState, EvolutionTrainingEvent},
    GroundMarker,
};


#[derive(Resource)]
pub struct GenerationTestingConfig {
    /// The number of physics time steps to test the creature for
    pub test_time: usize,
    pub session: String,
    pub wait_for_fall: bool,
    pub wait_for_fall_timeout: usize,
}

impl Default for GenerationTestingConfig {
    fn default() -> Self {
        Self { test_time: 180, session: String::from("default-session"), wait_for_fall: false, wait_for_fall_timeout: 300 }
    }
}


#[derive(Resource, Default)]
pub struct EvolutionGeneration<F: EvolutionFitnessEval + Send + Sync + Default + 'static> {
    pub(crate) population: Vec<CreatureMorphologyGraph>,
    pub(crate) fitnesses: Vec<f32>,
    pub(crate) populate_flags: Vec<CreaturePopulateFlag>,
    pub(crate) current_test: Option<usize>,
    pub(crate) current_fitness: Option<F>,
    pub(crate) current_train_time: usize,
    pub(crate) current_creature: Option<CreatureId>,
    pub(crate) current_generation: usize,
    pub(crate) waiting_for_fall: bool,
    pub(crate) fall_wait_time: usize,
    pub(crate) fall_start_counter: usize,
}


pub(crate) fn test_generation_nowindow<F: EvolutionFitnessEval + Send + Sync + Default + 'static>(
    mut commands: Commands,
    mut generation: ResMut<EvolutionGeneration<F>>,
    config: Res<GenerationTestingConfig>,
    state: Res<State<EvolutionState>>,
    mut next_state: ResMut<NextState<EvolutionState>>,
    mut limbs: Query<(Entity, &CreatureLimb, &Transform, &Velocity, &mut Friction, &mut Restitution)>,
    mut ground: Query<&mut Friction, (With<GroundMarker>, Without<CreatureLimb>)>,
    mut training_evw: EventWriter<EvolutionTrainingEvent>,
    mut build_conf: ResMut<CreatureBuilderConfig>,
    mut limb_info_save: Local<HashMap<Entity, (f32, f32)>>,
) {
    match state.get() {
        EvolutionState::EvaluatingCreature => {
            match generation.current_test {
                Some(i) => {
                    let limb_pos_vels: Vec<_> = limbs
                        .iter()
                        .filter(|(_, limb, _, _, _, _)| limb.creature == generation.population[generation.current_test.unwrap()].creature)
                        .map(|(_, _, pos, vel, _, _)| (*pos, *vel))
                        .collect();

                    let eval = generation
                        .current_fitness
                        .as_ref()
                        .unwrap()
                        .final_eval(FitnessEvalInput { limbs: limb_pos_vels, test_time: config.test_time });
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
                if let Some(id) = generation.current_creature {
                    limbs
                        .iter()
                        .filter(|(_, limb, _, _, _, _)| limb.creature == id)
                        .for_each(|(entity, _, _, _, _, _)| commands.entity(entity).despawn());
                }
                let morph = &generation.population[generation.current_test.unwrap()];
                let mut result = morph.evaluate();
                generation.current_creature = Some(morph.creature);
                result.align_to_ground();
                result.build_nowindow(&mut commands);
                if config.wait_for_fall {
                    build_conf.behavior.disable_behavior = true;
                    generation.waiting_for_fall = true;
                    for (entity, _, _, _, mut friction, mut restitution) in limbs.iter_mut() {
                        limb_info_save.insert(entity, (friction.coefficient, restitution.coefficient));
                        friction.coefficient = 0.0;
                        restitution.coefficient = 0.0;
                    }
                    for mut friction in ground.iter_mut() {
                        friction.coefficient = 0.0;
                    }
                }
                next_state.set(EvolutionState::TestingCreature);
            } else {
                generation.current_test = None;
                generation.current_fitness = None;
                next_state.set(EvolutionState::WritingGeneration);
            }
        },
        EvolutionState::TestingCreature => {
            let index = generation.current_test.unwrap();
            let morph = &generation.population[index];
            let creature_id = morph.creature;

            let limb_pos_vels: Vec<_> = limbs
                .iter()
                .filter(|(_, limb, _, _, _, _)| limb.creature == creature_id)
                .map(|(_, _, pos, vel, _, _)| (*pos, *vel))
                .collect();

            if generation.waiting_for_fall {
                generation.fall_wait_time += 1;
                if generation.fall_start_counter < 30 {
                    generation.fall_start_counter += 1;
                } else {
                    let mut y_vel = 0.0;
                    limb_pos_vels.iter().for_each(|x| {
                        y_vel += x.1.linvel.y;
                    });
                    if y_vel.abs() < 0.01 || generation.fall_wait_time > config.wait_for_fall_timeout {
                        generation.waiting_for_fall = false;
                        build_conf.behavior.disable_behavior = false;
                        generation.fall_wait_time = 0;
                        generation.fall_start_counter = 0;

                        for (entity, _, _, _, mut friction, mut restitution) in limbs.iter_mut() {
                            let Some((f, r)) = limb_info_save.get(&entity) else { continue };
                            friction.coefficient = *f;
                            restitution.coefficient = *r;
                        }
                        for mut friction in ground.iter_mut() {
                            friction.coefficient = 0.75;
                        }
                        limb_info_save.clear();

                        generation
                            .current_fitness
                            .as_mut()
                            .unwrap()
                            .eval_start(FitnessEvalInput { limbs: limb_pos_vels, test_time: config.test_time });
                    }
                }
                return;
            }

            generation.current_train_time += 1;

            generation
                .current_fitness
                .as_mut()
                .unwrap()
                .eval_continuous(FitnessEvalInput { limbs: limb_pos_vels, test_time: config.test_time });

            if generation.current_train_time > config.test_time {
                generation.current_train_time = 0;
                training_evw.send(EvolutionTrainingEvent::FinishedTestingCreature);
                next_state.set(EvolutionState::EvaluatingCreature);
            }
        },

        _ => (),
    }
}


pub(crate) fn test_generation<F: EvolutionFitnessEval + Send + Sync + Default + 'static>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut generation: ResMut<EvolutionGeneration<F>>,
    config: Res<GenerationTestingConfig>,
    state: Res<State<EvolutionState>>,
    mut next_state: ResMut<NextState<EvolutionState>>,
    mut limbs: Query<(Entity, &CreatureLimb, &Transform, &Velocity, &mut Friction, &mut Restitution)>,
    mut ground: Query<&mut Friction, (With<GroundMarker>, Without<CreatureLimb>)>,
    mut training_evw: EventWriter<EvolutionTrainingEvent>,
    mut build_conf: ResMut<CreatureBuilderConfig>,
    mut limb_info_save: Local<HashMap<Entity, (f32, f32)>>,
) {
    match state.get() {
        EvolutionState::EvaluatingCreature => {
            match generation.current_test {
                Some(i) => {
                    let limb_pos_vels: Vec<_> = limbs
                        .iter()
                        .filter(|(_, limb, _, _, _, _)| limb.creature == generation.population[generation.current_test.unwrap()].creature)
                        .map(|(_, _, pos, vel, _, _)| (*pos, *vel))
                        .collect();

                    let eval = generation
                        .current_fitness
                        .as_ref()
                        .unwrap()
                        .final_eval(FitnessEvalInput { limbs: limb_pos_vels, test_time: config.test_time });
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
                if let Some(id) = generation.current_creature {
                    limbs
                        .iter()
                        .filter(|(_, limb, _, _, _, _)| limb.creature == id)
                        .for_each(|(entity, _, _, _, _, _)| commands.entity(entity).despawn());
                }
                let morph = &generation.population[generation.current_test.unwrap()];
                let mut result = morph.evaluate();
                generation.current_creature = Some(morph.creature);
                result.align_to_ground();
                result.build(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    generation.populate_flags[generation.current_test.unwrap()].into_color(),
                );
                if config.wait_for_fall {
                    build_conf.behavior.disable_behavior = true;
                    generation.waiting_for_fall = true;
                    for (entity, _, _, _, mut friction, mut restitution) in limbs.iter_mut() {
                        limb_info_save.insert(entity, (friction.coefficient, restitution.coefficient));
                        friction.coefficient = 0.0;
                        restitution.coefficient = 0.0;
                    }
                    for mut friction in ground.iter_mut() {
                        friction.coefficient = 0.0;
                    }
                }
                next_state.set(EvolutionState::TestingCreature);
            } else {
                generation.current_test = None;
                generation.current_fitness = None;
                training_evw.send(EvolutionTrainingEvent::StartTestingGeneration(generation.current_generation));
                next_state.set(EvolutionState::WritingGeneration);
            }
        },
        EvolutionState::TestingCreature => {
            let index = generation.current_test.unwrap();
            let morph = &generation.population[index];
            let creature_id = morph.creature;

            let limb_pos_vels: Vec<_> = limbs
                .iter()
                .filter(|(_, limb, _, _, _, _)| limb.creature == creature_id)
                .map(|(_, _, pos, vel, _, _)| (*pos, *vel))
                .collect();

            if generation.waiting_for_fall {
                generation.fall_wait_time += 1;
                if generation.fall_start_counter < 30 {
                    generation.fall_start_counter += 1;
                } else {
                    let mut y_vel = 0.0;
                    limb_pos_vels.iter().for_each(|x| {
                        y_vel += x.1.linvel.y;
                    });
                    if y_vel.abs() < 0.01 || generation.fall_wait_time > config.wait_for_fall_timeout {
                        generation.waiting_for_fall = false;
                        build_conf.behavior.disable_behavior = false;
                        generation.fall_wait_time = 0;
                        generation.fall_start_counter = 0;

                        for (entity, _, _, _, mut friction, mut restitution) in limbs.iter_mut() {
                            let Some((f, r)) = limb_info_save.get(&entity) else { continue };
                            friction.coefficient = *f;
                            restitution.coefficient = *r;
                        }
                        for mut friction in ground.iter_mut() {
                            friction.coefficient = 0.75;
                        }
                        limb_info_save.clear();

                        generation
                            .current_fitness
                            .as_mut()
                            .unwrap()
                            .eval_start(FitnessEvalInput { limbs: limb_pos_vels, test_time: config.test_time });
                    }
                }
                return;
            }

            generation.current_train_time += 1;

            generation
                .current_fitness
                .as_mut()
                .unwrap()
                .eval_continuous(FitnessEvalInput { limbs: limb_pos_vels, test_time: config.test_time });

            if generation.current_train_time > config.test_time {
                generation.current_train_time = 0;
                training_evw.send(EvolutionTrainingEvent::FinishedTestingCreature);
                next_state.set(EvolutionState::EvaluatingCreature);
            }
        },

        _ => (),
    }
}
