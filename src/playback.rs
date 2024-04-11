use std::time::{Duration, Instant};

use behavior_evolver::evolution::{write, CreatureEnvironmentPlugin};
use bevy::prelude::*;
use creature_builder::{builder::node::CreatureMorphologyGraph, limb::CreatureLimb};

pub enum PlaybackMode {
    Creature(usize),
    Generation(usize),
}

#[derive(Resource)]
pub struct PlaybackConfig {
    pub session: String,
    pub mode: PlaybackMode,
    pub auto_cycle: Option<Duration>,
}

impl Default for PlaybackConfig {
    fn default() -> Self {
        Self { session: String::from("default-session"), mode: PlaybackMode::Creature(0), auto_cycle: None }
    }
}


pub fn play(conf: PlaybackConfig) {
    let mut app = App::new();
    app.insert_resource(conf);
    app.add_systems(Startup, setup);
    app.add_systems(Update, cycle_creature);
    app.add_plugins(DefaultPlugins).add_plugins(CreatureEnvironmentPlugin { window: true });
    app.run();
}


#[derive(Resource)]
struct PlaybackCreatures(Vec<CreatureMorphologyGraph>, usize, Instant);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    conf: Res<PlaybackConfig>,
) {
    match conf.mode {
        PlaybackMode::Creature(id) => {
            let morph = write::load_creature(&conf.session, id);
            let mut res = morph.evaluate();
            res.align_to_ground();
            res.build(&mut commands, &mut meshes, &mut materials, Color::rgba_u8(137, 220, 235, 220));
            commands.insert_resource(PlaybackCreatures(vec![morph], 0, Instant::now()));
        },
        PlaybackMode::Generation(id) => {
            let morphs = write::load_generation(&conf.session, id);
            let mut res = morphs[0].evaluate();
            res.align_to_ground();
            res.build(&mut commands, &mut meshes, &mut materials, Color::rgba_u8(137, 220, 235, 220));
            commands.insert_resource(PlaybackCreatures(morphs, 0, Instant::now()));
        },
    }
}


fn cycle_creature(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut creatures: ResMut<PlaybackCreatures>,
    limbs: Query<Entity, With<CreatureLimb>>,
    keys: Res<Input<KeyCode>>,
    conf: Res<PlaybackConfig>,
) {
    let cycle_switch = match conf.auto_cycle {
        Some(cycle_delay) => Instant::now().duration_since(creatures.2) > cycle_delay,
        None => false,
    };

    if keys.just_pressed(KeyCode::Space) || cycle_switch {
        limbs.for_each(|entity| commands.entity(entity).despawn());
        creatures.1 = (creatures.1 + 1).rem_euclid(creatures.0.len());
        let morph = &creatures.0[creatures.1];
        let mut res = morph.evaluate();
        res.align_to_ground();
        res.build(&mut commands, &mut meshes, &mut materials, Color::rgba_u8(137, 220, 235, 220));
    }
}
