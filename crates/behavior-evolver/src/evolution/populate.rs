use bevy::prelude::*;
use creature_builder::CreatureId;

use super::{fitness::EvolutionFitnessEval, generation::EvolutionGeneration, state::EvolutionState};
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
    current_id: usize,
}

impl Default for GenerationPopulator {
    fn default() -> Self {
        Self {
            elitism: 0.3,
            rand_percent: 0.2,
            pop_size: 10,
            mutate_params: MutateMorphologyParams::default(),
            rand_params: RandomMorphologyParams::default(),
            current_id: 0,
        }
    }
}


pub(crate) fn populate_generation<F: EvolutionFitnessEval + Send + Sync + Default + 'static>(
    mut generation: ResMut<EvolutionGeneration<F>>,
    mut populator: ResMut<GenerationPopulator>,
    mut next_state: ResMut<NextState<EvolutionState>>,
) {
    if generation.population.is_empty() {
        info!("initializing generation...");
        let mut rng = rand::thread_rng();
        for i in 0..populator.pop_size {
            generation.population.push(populator.rand_params.build_morph(&mut rng, CreatureId(i)));
            populator.current_id += 1;
        }
        next_state.set(EvolutionState::EvaluatingCreature);
        return;
    }

    info!("building next generation");
    let mut elite: Vec<_> = generation.population.iter().enumerate().collect();
    elite.sort_unstable_by(|(i, _), (j, _)| {
        (-generation.fitnesses[*i])
            .partial_cmp(&-generation.fitnesses[*j])
            .expect("Unable to sort generation, fitnesses likely contains NAN values")
    });

    let retained = (populator.elitism * populator.pop_size as f32).ceil() as usize;
    let rand_amt = (populator.rand_percent * populator.pop_size as f32).ceil() as usize;

    elite.truncate(retained);
    generation.population = elite.iter().map(|(_, x)| (*x).clone()).collect();
    generation.fitnesses.clear();

    let mut rng = rand::thread_rng();
    let mut params = populator.mutate_params.clone();
    for i in 0..populator.pop_size - retained - rand_amt {
        let mut morph = generation.population[i % retained].clone();
        morph.creature = CreatureId(populator.current_id);
        populator.current_id += 1;
        let mut mutate = MutateMorphology::new(&mut morph, &mut rng, &mut params);
        mutate.mutate();
        generation.population.push(morph);
    }

    for _ in 0..retained {
        generation.population.push(populator.rand_params.build_morph(&mut rng, CreatureId(populator.current_id)));
        populator.current_id += 1;
    }

    println!("{}", generation.population.len());

    next_state.set(EvolutionState::EvaluatingCreature);
}
