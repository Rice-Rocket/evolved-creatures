use behavior_evolver::{
    evolution::CreatureEnvironmentPlugin,
    mutate::{MutateMorphology, MutateMorphologyParams, RandomMorphologyParams},
};
use bevy::prelude::*;
use creature_builder::{builder::node::CreatureMorphologyGraph, limb::CreatureLimb, CreatureId};

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
        .insert_resource(CurrentCreatureInfo(CreatureMorphologyGraph::new(CreatureId(0))))
        .run()
}

fn setup() {
    // state.set(EvolutionState::PopulatingGeneration);
}

#[derive(Resource)]
struct CurrentCreatureInfo(CreatureMorphologyGraph);

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
        res.build(&mut commands, &mut meshes, &mut materials);
        commands.insert_resource(CurrentCreatureInfo(morph));
    }

    if keys.just_pressed(KeyCode::P) {
        for edge in current.0.edges() {
            edge.data.effectors.effectors.iter().filter(|x| x.is_some()).for_each(|x| println!("\n{:?}", x.as_ref().unwrap().expr.root));
            println!("\n-----------");
        }
    }
}
