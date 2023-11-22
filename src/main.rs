use bevy::prelude::*;
use body::SoftBody;
use draw::{draw_particles, setup_gizmo_config, draw_springs};
use particle::{apply_particle_gravity, apply_collision, update_particle_positions, update_particle_velocities, ParticleAccelerateSet, ParticleProperties};
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;

pub mod particle;
pub mod draw;
pub mod collision;
pub mod body;
pub mod spring;
pub mod sim;

use collision::*;
use sim::{PhysicsSimulationSchedule, run_physics_sim_schedule, PhysicsSimulationSettings};
use spring::{SpringProperties, apply_spring_force};





fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)

        .add_systems(Startup, setup_gizmo_config)
        .add_systems(Update, (draw_particles, draw_springs))

        .init_resource::<PhysicsSimulationSettings>()
        .add_schedule(Schedule::new(PhysicsSimulationSchedule))
        
        .add_systems(Update, run_physics_sim_schedule)
        .add_systems(PhysicsSimulationSchedule, (
            update_particle_positions.before(ParticleAccelerateSet),
            update_particle_velocities.after(ParticleAccelerateSet),
        ))
        .add_systems(PhysicsSimulationSchedule, (
            apply_particle_gravity,
            apply_spring_force,
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
            IVec2::new(30, 15), 
            Transform::from_xyz(0.0, 100.0, 0.0).with_scale(Vec3::new(300.0, 150.0, 0.0))
        )
        .with_particle_properties(ParticleProperties {
            mass: 10.0,
            restitution: 0.5,
        })
        .with_spring_properties(SpringProperties {
            stiffness: 1000.0,
            damping: 50.0,
            ..default()
        })
        .tesselate_from_dims(IVec2::new(10, 5))
        .set_spring_rest_lengths()
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