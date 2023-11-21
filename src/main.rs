use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use particle::{integrate_particles, Particle, apply_particle_gravity};

pub mod particle;
pub mod draw;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)

        .add_systems(Update, (
            apply_particle_gravity.before(integrate_particles),
            integrate_particles,
        ))

        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        Particle {
            position_old: Vec3::new(0.0, 0.0, 0.0),
            acceleration: Vec3::ZERO,
        },
    ));
}