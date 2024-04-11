use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use behavior_evolver::evolution::{write, CreatureEnvironmentPlugin, GroundMarker};
use bevy::prelude::*;
use bevy_rapier3d::{
    dynamics::Velocity,
    geometry::{Friction, Restitution},
};
use creature_builder::{builder::node::CreatureMorphologyGraph, config::CreatureBuilderConfig, limb::CreatureLimb};

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
    App::new()
        .insert_resource(conf)
        .insert_resource(PlaybackCreatures(Vec::new(), 0, Instant::now(), true))
        .add_systems(Startup, setup)
        .add_systems(Update, cycle_creature)
        .add_plugins(DefaultPlugins)
        .add_plugins(CreatureEnvironmentPlugin { window: true })
        .run();
}


#[derive(Resource)]
struct PlaybackCreatures(Vec<CreatureMorphologyGraph>, usize, Instant, bool);

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
            commands.insert_resource(PlaybackCreatures(vec![morph], 0, Instant::now(), true));
            commands.insert_resource(WaitingForFall(false));
        },
        PlaybackMode::Generation(id) => {
            let morphs = write::load_generation(&conf.session, id);
            let mut res = morphs[0].evaluate();
            res.align_to_ground();
            res.build(&mut commands, &mut meshes, &mut materials, Color::rgba_u8(137, 220, 235, 220));
            commands.insert_resource(PlaybackCreatures(morphs, 0, Instant::now(), true));
            commands.insert_resource(WaitingForFall(false));
        },
    }
}


#[derive(Resource)]
struct WaitingForFall(bool);

fn cycle_creature(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut creatures: ResMut<PlaybackCreatures>,
    mut limbs: Query<(Entity, &Velocity, &mut Friction, &mut Restitution), With<CreatureLimb>>,
    mut ground: Query<&mut Friction, (With<GroundMarker>, Without<CreatureLimb>)>,
    keys: Res<Input<KeyCode>>,
    conf: Res<PlaybackConfig>,
    mut waiting_for_fall: ResMut<WaitingForFall>,
    mut limb_info_save: Local<HashMap<Entity, (f32, f32)>>,
    mut build_conf: ResMut<CreatureBuilderConfig>,
    time: Res<Time>,
) {
    if creatures.3 {
        creatures.3 = false;
        waiting_for_fall.0 = true;
        build_conf.behavior.disable_behavior = true;
        for (entity, _, mut friction, mut restitution) in limbs.iter_mut() {
            limb_info_save.insert(entity, (friction.coefficient, restitution.coefficient));
            friction.coefficient = 0.0;
            restitution.coefficient = 0.0;
        }
        for mut friction in ground.iter_mut() {
            friction.coefficient = 0.0;
        }
    }

    let cycle_switch = match conf.auto_cycle {
        Some(cycle_delay) => Instant::now().duration_since(creatures.2) > cycle_delay,
        None => false,
    };

    if waiting_for_fall.0 && time.elapsed_seconds() > 0.5 {
        let mut y_vel = 0.0;
        limbs.iter().for_each(|x| y_vel += x.1.linvel.y);
        if y_vel.abs() < 0.01 {
            waiting_for_fall.0 = false;
            build_conf.behavior.disable_behavior = false;
            for (entity, _, mut friction, mut restitution) in limbs.iter_mut() {
                let Some((f, r)) = limb_info_save.get(&entity) else { continue };
                friction.coefficient = *f;
                restitution.coefficient = *r;
            }
            for mut friction in ground.iter_mut() {
                friction.coefficient = 0.3;
            }
            limb_info_save.clear();
        }
    }

    if keys.just_pressed(KeyCode::Space) || cycle_switch {
        limbs.iter().for_each(|(entity, _, _, _)| commands.entity(entity).despawn());
        creatures.1 = (creatures.1 + 1).rem_euclid(creatures.0.len());
        let morph = &creatures.0[creatures.1];
        let mut res = morph.evaluate();
        res.align_to_ground();
        res.build(&mut commands, &mut meshes, &mut materials, Color::rgba_u8(137, 220, 235, 220));
        waiting_for_fall.0 = true;
        build_conf.behavior.disable_behavior = true;
        for (entity, _, mut friction, mut restitution) in limbs.iter_mut() {
            limb_info_save.insert(entity, (friction.coefficient, restitution.coefficient));
            friction.coefficient = 0.0;
            restitution.coefficient = 0.0;
        }
        for mut friction in ground.iter_mut() {
            friction.coefficient = 0.0;
        }
    }
}
