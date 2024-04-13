use bevy::prelude::*;
use creature_builder::CreatureId;
use serde::{Deserialize, Serialize};

use super::{
    fitness::EvolutionFitnessEval,
    generation::EvolutionGeneration,
    state::{EvolutionState, EvolutionTrainingEvent},
};
use crate::mutate::{MutateMorphology, MutateMorphologyParams, RandomMorphologyParams};


#[derive(Resource)]
pub struct GenerationPopulator {
    /// The portion of the previous generation's population that is preserved
    /// and mutated to make up the next generation
    pub elitism: f32,
    /// The percentage of the new population that is composed of new random
    /// creatures
    pub rand_percent: f32,
    /// The size of a generation
    pub pop_size: usize,
    pub mutate_params: MutateMorphologyParams,
    pub rand_params: RandomMorphologyParams,
    pub current_id: usize,
    pub best_fitness: f32,
    pub best_creature: usize,
    pub num_mutations: usize,
}

impl GenerationPopulator {
    pub fn new(
        elitism: f32,
        rand_percent: f32,
        pop_size: usize,
        mutate_params: MutateMorphologyParams,
        rand_params: RandomMorphologyParams,
        num_mutations: usize,
    ) -> Self {
        Self {
            elitism,
            rand_percent,
            pop_size,
            mutate_params,
            rand_params,
            current_id: 0,
            best_fitness: -1000000000000.0,
            best_creature: 0,
            num_mutations,
        }
    }
}

impl Default for GenerationPopulator {
    fn default() -> Self {
        Self {
            elitism: 0.25,
            rand_percent: 0.03,
            pop_size: 100,
            mutate_params: MutateMorphologyParams::default(),
            rand_params: RandomMorphologyParams::default(),
            current_id: 0,
            best_fitness: 0.0,
            best_creature: 0,
            num_mutations: 80,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum CreaturePopulateFlag {
    Retained,
    Mutated,
    Spawned,
}

impl CreaturePopulateFlag {
    pub fn into_color(&self) -> Color {
        match self {
            Self::Retained => Color::rgba_u8(166, 227, 161, 220),
            Self::Mutated => Color::rgba_u8(137, 220, 235, 220),
            Self::Spawned => Color::rgba_u8(249, 226, 175, 220),
        }
    }
}


pub(crate) fn populate_generation<F: EvolutionFitnessEval + Send + Sync + Default + 'static>(
    mut generation: ResMut<EvolutionGeneration<F>>,
    mut populator: ResMut<GenerationPopulator>,
    mut next_state: ResMut<NextState<EvolutionState>>,
    mut training_evw: EventWriter<EvolutionTrainingEvent>,
) {
    if generation.population.is_empty() {
        let mut rng = rand::thread_rng();
        for i in 0..populator.pop_size {
            generation.population.push(populator.rand_params.build_morph(&mut rng, CreatureId(i)));
            generation.populate_flags.push(CreaturePopulateFlag::Spawned);
            populator.current_id += 1;
        }
        next_state.set(EvolutionState::EvaluatingCreature);
        training_evw.send(EvolutionTrainingEvent::StartTestingGeneration(generation.current_generation));
        return;
    }

    generation.current_generation += 1;

    let mut elite: Vec<_> = generation.population.iter().enumerate().collect();
    elite.sort_unstable_by(|(i, _), (j, _)| {
        (-generation.fitnesses[*i])
            .partial_cmp(&-generation.fitnesses[*j])
            .expect("Unable to sort generation, fitnesses likely contains NAN values")
    });

    let retained = (populator.elitism * populator.pop_size as f32).ceil() as usize;
    let rand_amt = (populator.rand_percent * populator.pop_size as f32).ceil() as usize;

    populator.best_fitness = generation.fitnesses[elite[0].0];
    populator.best_creature = elite[0].1.creature.0;

    elite.truncate(retained);
    generation.population = elite.iter().map(|(_, x)| (*x).clone()).collect();
    generation.fitnesses.clear();

    generation.populate_flags.clear();
    for _ in 0..generation.population.len() {
        generation.populate_flags.push(CreaturePopulateFlag::Retained);
    }

    let mut rng = rand::thread_rng();
    let mut params = populator.mutate_params.clone();
    let mutate_amt = populator.pop_size - retained - rand_amt;
    for i in 0..mutate_amt {
        let mut morph = generation.population[i % retained].clone();
        morph.creature = CreatureId(populator.current_id);
        populator.current_id += 1;
        let mut mutate = MutateMorphology::new(&mut morph, &mut rng, &mut params);

        let n_mutations = (populator.num_mutations as f32 * (i as f32 / (mutate_amt - 1) as f32).powf(2.4)).ceil() as usize;
        for _ in 0..n_mutations {
            mutate.mutate();
        }

        generation.population.push(morph);
        generation.populate_flags.push(CreaturePopulateFlag::Mutated);
    }

    for _ in 0..rand_amt {
        generation.population.push(populator.rand_params.build_morph(&mut rng, CreatureId(populator.current_id)));
        generation.populate_flags.push(CreaturePopulateFlag::Spawned);
        populator.current_id += 1;
    }

    training_evw.send(EvolutionTrainingEvent::StartTestingGeneration(generation.current_generation));
    next_state.set(EvolutionState::EvaluatingCreature);
}
