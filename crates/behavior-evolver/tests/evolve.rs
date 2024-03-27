use behavior_evolver::evolution::{fitness::jump::JumpFitnessEval, CreatureEvolutionPlugin};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CreatureEvolutionPlugin::<JumpFitnessEval>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run()
}


fn setup() {}

fn update() {}
