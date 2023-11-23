use bevy::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

pub mod particle;
pub mod draw;
pub mod collision;
pub mod body;
pub mod spring;
pub mod sim;

use collision::*;
use sim::*;
use spring::*;
use particle::*;
use draw::*;
use body::{*, standard::*, constrained::*};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)

        .add_systems(Startup, setup_gizmo_config)
        .add_systems(Update, (draw_particles, draw_springs, draw_colliders))

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
            apply_constraint_force,
            apply_collision::<HalfSpace>,
            apply_collision::<StaticPolygon>,
        ).in_set(ParticleAccelerateSet).after(update_particle_positions))

        .register_type::<ColliderProperties>()
        .register_type::<HalfSpace>()
        .register_type::<SoftBodyMassPoints>()
        .register_type::<SoftBodySprings>()
        .register_type::<PhysicsSimulationSettings>()

        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(ResourceInspectorPlugin::<PhysicsSimulationSettings>::default())

        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(
        ConstrainedSoftBody::rect(
            IVec2::new(20, 10), 
            Transform::from_xyz(0.0, 300.0, 0.0).with_scale(Vec3::new(150.0, 75.0, 0.0))
        )
        .with_properties(ConstraintProperties {
            stiffness: 50000.0,
            damping: 20.0,
        })
        .with_particle_properties(ParticleProperties {
            mass: 10.0,
            restitution: 0.5,
        })
        .with_spring_properties(SpringProperties {
            stiffness: 60000.0,
            damping: 20.0,
            ..default()
        })
        .tesselate_from_dims(IVec2::new(20, 10))
        .set_spring_rest_lengths()
    );

    // commands.spawn(
    //     StandardSoftBody::rect(
    //         IVec2::new(30, 15), 
    //         Transform::from_xyz(0.0, 100.0, 0.0).with_scale(Vec3::new(300.0, 150.0, 0.0))
    //     )
    //     .with_particle_properties(ParticleProperties {
    //         mass: 10.0,
    //         restitution: 0.5,
    //     })
    //     .with_spring_properties(SpringProperties {
    //         stiffness: 60000.0,
    //         damping: 50.0,
    //         ..default()
    //     })
    //     .tesselate_from_dims(IVec2::new(20, 10))
    //     .set_spring_rest_lengths()
    // );

    let collider_props = ColliderProperties {
        elasticity: 20000.0,
        friction: 300.0,
        restitution: 100.0,
    };

    commands.spawn((
        HalfSpace {
            normal: Vec3::new(0.0, 1.0, 0.0).normalize(),
            k: -300.0,
        },
        collider_props.clone(),
    ));

    commands.spawn((
        StaticPolygon::new_square()
            .with_transform(
                Transform::from_xyz(-150.0, -80.0, 0.0)
                .with_scale(Vec3::new(200.0, 100.0, 1.0))
                .with_rotation(Quat::from_euler(EulerRot::ZXY, 2.6, 0.0, 0.0))
            ),
        collider_props.clone(),
    ));

    commands.spawn((
        StaticPolygon::from_vertices(vec![
            Vec3::new(-0.5, -0.5, 0.0), Vec3::new(0.5, -0.5, 0.0), 
            Vec3::new(0.4, 0.5, 0.0), Vec3::new(0.0, -0.2, 0.0), 
            Vec3::new(-0.4, 0.5, 0.0), 
        ]).with_transform(
            Transform::from_xyz(150.0, -150.0, 0.0)
            .with_scale(Vec3::new(200.0, 100.0, 1.0))
            .with_rotation(Quat::from_euler(EulerRot::ZXY, 0.5, 0.0, 0.0))
        ),
        collider_props.clone(),
    ));
}