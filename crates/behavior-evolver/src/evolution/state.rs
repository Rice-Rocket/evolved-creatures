use bevy::prelude::*;

use super::{
    fitness::EvolutionFitnessEval,
    generation::{EvolutionGeneration, GenerationTestingConfig},
    populate::GenerationPopulator,
    write::load_session,
};


#[derive(States, Clone, Hash, PartialEq, Eq, Debug, Default)]
pub enum EvolutionState {
    BeginTrainingSession,
    TestingCreature,
    EvaluatingCreature,
    WritingGeneration,
    PopulatingGeneration,
    #[default]
    None,
}


#[derive(Event)]
pub enum EvolutionTrainingEvent {
    FinishedTestingCreature,
    FinishedTestingGeneration(usize),
}


pub fn begin_training_session<F: EvolutionFitnessEval + Send + Sync + Default + 'static>(
    mut next_state: ResMut<NextState<EvolutionState>>,
    mut generation: ResMut<EvolutionGeneration<F>>,
    mut populator: ResMut<GenerationPopulator>,
    gen_test_conf: Res<GenerationTestingConfig>,
) {
    load_session(generation.as_mut(), populator.as_mut(), gen_test_conf.as_ref());
    next_state.set(EvolutionState::PopulatingGeneration);
}
