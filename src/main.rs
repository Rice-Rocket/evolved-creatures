use bevy::prelude::*;
use body::SoftBody;
use draw::{draw_particles, setup_gizmo_config};
use particle::{apply_particle_gravity, apply_collision, update_particle_positions, update_particle_velocities, ParticleAccelerateSet, ParticleProperties};
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;

pub mod particle;
pub mod draw;
pub mod collision;
pub mod body;

use collision::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)

        .add_systems(Startup, setup_gizmo_config)
        .add_systems(Update, draw_particles)
        .add_systems(Update, (
            update_particle_positions.before(ParticleAccelerateSet),
            update_particle_velocities.after(ParticleAccelerateSet),
        ))
        .add_systems(Update, (
            apply_particle_gravity,
            apply_collision::<HalfSpace>,
        ).in_set(ParticleAccelerateSet))

        .register_type::<ColliderProperties>()
        .register_type::<HalfSpace>()

        .add_plugins(FilterQueryInspectorPlugin::<With<ColliderProperties>>::default())

        .run();
}


fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(
        SoftBody::rect(
            IVec2::new(20, 3), 
            Transform::from_xyz(0.0, 100.0, 0.0).with_scale(Vec3::new(1000.0, 200.0, 0.0))
        )
        .with_particle_properties(ParticleProperties {
            mass: 10.0,
            restitution: 0.5,
        })
    );

    commands.spawn((
        HalfSpace {
            normal: Vec3::new(0.2, 1.0, 0.0).normalize(),
            k: -200.0,
        },
        ColliderProperties {
            elasticity: 20000.0,
            friction: 1.0,
            restitution: 100.0,
        }
    ));
}