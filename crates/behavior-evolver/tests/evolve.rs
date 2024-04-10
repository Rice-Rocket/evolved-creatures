use behavior_evolver::{
    evolution::{
        fitness::jump::JumpFitnessEval, generation::GenerationTestingConfig, populate::GenerationPopulator, state::EvolutionState,
        CreatureEvolutionPlugin,
    },
    mutate::{MutateMorphology, MutateMorphologyParams, RandomMorphologyParams},
};
use bevy::prelude::*;
use creature_builder::{builder::node::CreatureMorphologyGraph, limb::CreatureLimb, CreatureId};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { present_mode: bevy::window::PresentMode::AutoNoVsync, ..default() }),
            ..default()
        }))
        .add_plugins(CreatureEvolutionPlugin::<JumpFitnessEval>::default())
        // .add_plugins(CreatureEnvironmentPlugin)
        .add_systems(Startup, setup)
        // .add_systems(Update, update)
        .insert_resource(CurrentCreatureInfo(CreatureMorphologyGraph::new(CreatureId(0))))
        .run()
}

fn setup(mut commands: Commands, mut state: ResMut<NextState<EvolutionState>>) {
    commands.insert_resource(GenerationTestingConfig { test_time: 180, session: String::from("default-session") });
    commands.insert_resource(GenerationPopulator::new(0.3, 0.2, 10, MutateMorphologyParams::default(), RandomMorphologyParams::default()));
    state.set(EvolutionState::BeginTrainingSession);
}

#[derive(Resource)]
struct CurrentCreatureInfo(CreatureMorphologyGraph);

#[allow(dead_code)]
fn update(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keys: Res<Input<KeyCode>>,
    limbs: Query<Entity, With<CreatureLimb>>,
    current: Res<CurrentCreatureInfo>,
) {
    if keys.just_pressed(KeyCode::Space) {
        limbs.for_each(|entity| commands.entity(entity).despawn());
        let mut rng = rand::thread_rng();
        let mut morph = RandomMorphologyParams::default().build_morph(&mut rng, CreatureId(0));
        let mut params = MutateMorphologyParams::default();
        let mut mutate = MutateMorphology::new(&mut morph, &mut rng, &mut params);
        mutate.mutate();
        mutate.mutate();
        mutate.mutate();
        mutate.mutate();
        mutate.mutate();
        mutate.mutate();
        mutate.mutate();
        mutate.mutate();
        mutate.mutate();
        mutate.mutate();
        mutate.mutate();
        mutate.mutate();
        mutate.mutate();
        mutate.mutate();
        let mut res = morph.evaluate();
        res.align_to_ground();
        res.build(&mut commands, &mut meshes, &mut materials, Color::rgba(1.0, 1.0, 1.0, 0.8));
        commands.insert_resource(CurrentCreatureInfo(morph));
    }

    if keys.just_pressed(KeyCode::P) {
        for edge in current.0.edges() {
            edge.data.effectors.effectors.iter().filter(|x| x.is_some()).for_each(|x| println!("\n{:?}", x.as_ref().unwrap().expr.root));
            println!("\n-----------");
        }
    }
}
