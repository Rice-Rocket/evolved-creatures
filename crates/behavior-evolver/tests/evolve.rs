use behavior_evolver::{evolution::CreatureEnvironmentPlugin, mutate::RandomMorphologyParams};
use bevy::prelude::*;
use creature_builder::{builder::node::BuildParameters, CreatureId};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        // .add_plugins(CreatureEvolutionPlugin::<JumpFitnessEval>::default())
        .add_plugins(CreatureEnvironmentPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run()
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let mut rng = rand::thread_rng();
    let morph = RandomMorphologyParams::default().build_morph(&mut rng, CreatureId(0));
    let mut res = morph.evaluate(BuildParameters { root_transform: Transform::from_xyz(0.0, 5.0, 0.0) });
    res.build(&mut commands, &mut meshes, &mut materials);
}

// fn setup() {
//     state.set(EvolutionState::PopulatingGeneration);
// }

fn update() {}
