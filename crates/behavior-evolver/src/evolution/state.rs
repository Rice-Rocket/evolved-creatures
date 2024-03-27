use bevy::prelude::*;


#[derive(States, Clone, Hash, PartialEq, Eq, Debug, Default)]
pub enum EvolutionState {
    TestingCreature,
    EvaluatingCreature,
    PopulatingGeneration,
    #[default]
    None,
}
