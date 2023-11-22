use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use particle::{ParticleTrajectory, apply_particle_gravity, apply_collision, update_particle_positions, update_particle_velocities, ParticleAccelerateSet, ParticleProperties, Particle};

pub mod particle;
pub mod draw;
pub mod collision;

use collision::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)

        .add_systems(Update, (
            update_particle_positions.before(ParticleAccelerateSet),
            update_particle_velocities.after(ParticleAccelerateSet),
        ))
        .add_systems(Update, (
            apply_particle_gravity,
            apply_collision::<HalfSpace>,
        ).in_set(ParticleAccelerateSet))

        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    for i in -10..10 {
        let pos = Vec3::new(i as f32, 0.0, 0.0) * 50.0;

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(pos),
                ..default()
            },
            Particle {
                properties: ParticleProperties {
                    mass: 10.0,
                    restitution: 0.5,
                },
                trajectory: ParticleTrajectory::default(),
            },
        ));
    }

    commands.spawn((
        HalfSpace {
            normal: Vec3::new(0.2, 1.0, 0.0).normalize(),
            k: -200.0,
        },
        ColliderProperties {
            elasticity: 20000.0,
            friction: 1.0,
            restitution: 1.0,
        }
    ));
}