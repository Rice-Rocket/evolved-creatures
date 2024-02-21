use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use bevy_editor_pls::prelude::*;

#[path = "./lib.rs"]
pub mod behavior_evolver;

use bevy_rapier3d::prelude::*;
use creature_builder::{builder::{node::{BuildParameters, CreatureMorphologyGraph, LimbConnection, LimbNode}, placement::{LimbAttachFace, LimbRelativePlacement}}, config::{ActiveCollisionTypes, CreatureBuilderConfig}, sensor::{ContactFilter, ContactFilterTag}, CreatureBuilderPlugin};


pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))

        .add_systems(Startup, (setup, behavior_evolver_scene, setup_ground))
        .add_plugins(CreatureBuilderPlugin)

        .add_plugins(RapierPhysicsPlugin::<ContactFilter>::default())
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(RapierPhysicsEditorPlugin)

        .insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Variable { max_dt: 1.0 / 60.0, time_scale: 1.0, substeps: 1 },
            gravity: Vec3::ZERO,
            ..default()
        })

        .insert_resource(CreatureBuilderConfig {
            collision_types: ActiveCollisionTypes::LIMB_VS_GROUND,
        })

        .run();
}


#[allow(dead_code)]
fn behavior_evolver_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    use std::f32::consts::FRAC_PI_2;

    let mut builder_graph = CreatureMorphologyGraph::new();

    let body = builder_graph.add_node(LimbNode {
        density: 1.0,
        terminal_only: false,
        recursive_limit: 1,
    });
    let arm = builder_graph.add_node(LimbNode {
        density: 1.0,
        terminal_only: false,
        recursive_limit: 2,
    });
    let arm2 = builder_graph.add_node(LimbNode {
        density: 1.0,
        terminal_only: false,
        recursive_limit: 2,
    });

    builder_graph.add_edge(LimbConnection {
        placement: LimbRelativePlacement {
            attach_face: LimbAttachFace::PosX,
            attach_position: Vec2::new(0.4, 0.9),
            orientation: Quat::from_euler(EulerRot::YXZ, 0.0, 0.2, 0.2),
            scale: Vec3::new(0.4, 0.8, 0.6),
        },
        locked_axes: LockedAxes::all(),
        limit_axes: [[1.0; 2]; 6],
    }, body, arm);
    builder_graph.add_edge(LimbConnection {
        placement: LimbRelativePlacement {
            attach_face: LimbAttachFace::PosX,
            attach_position: Vec2::new(0.4, -0.9),
            orientation: Quat::from_euler(EulerRot::YXZ, 0.0, -0.2, 0.2),
            scale: Vec3::new(0.4, 0.8, 0.6),
        },
        locked_axes: LockedAxes::all(),
        limit_axes: [[1.0; 2]; 6],
    }, body, arm);
    builder_graph.add_edge(LimbConnection {
        placement: LimbRelativePlacement {
            attach_face: LimbAttachFace::PosY,
            attach_position: Vec2::new(0.0, 0.0),
            orientation: Quat::from_euler(EulerRot::YXZ, 0.0, FRAC_PI_2, -FRAC_PI_2),
            scale: Vec3::new(1.0, 0.7, 1.0),
        },
        locked_axes: LockedAxes::all(),
        limit_axes: [[1.0; 2]; 6],
    }, arm, arm2);

    builder_graph.set_root(body);

    let mut result = builder_graph.evaluate(BuildParameters {
        root_transform: Transform::from_xyz(0.0, 5.0, 0.0).with_scale(Vec3::new(1.0, 1.5, 1.0)),
    });

    result.build(&mut commands, &mut meshes, &mut materials);
}


#[allow(dead_code)]
fn setup_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        RigidBody::KinematicPositionBased,
        Velocity { linvel: Vec3::ZERO, angvel: Vec3::ZERO },
        GravityScale(1.0),
        ActiveEvents::COLLISION_EVENTS,
        ActiveHooks::FILTER_CONTACT_PAIRS,
        ContactFilterTag::GroundGroup,

        Collider::cuboid(50.0, 5.0, 50.0),
        Friction { coefficient: 0.3, combine_rule: CoefficientCombineRule::Average },
        Restitution { coefficient: 0.0, combine_rule: CoefficientCombineRule::Average },
        ColliderMassProperties::Density(1.0),

        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(100.0, 10.0, 100.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.1, 0.1, 0.1),
                perceptual_roughness: 0.9,
                reflectance: 0.1,
                metallic: 0.0,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, -5.0, 0.0),
            ..default()
        },
        
        Name::new("Ground")
    ));
}


#[allow(dead_code)]
fn setup(
    mut gizmo_config: ResMut<GizmoConfig>,
    mut commands: Commands,
) {
    gizmo_config.depth_bias = -1.0;

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(5.0, 3.0, 4.0) * 8.0).looking_at(Vec3::new(0.0, 5.0, 0.0), Vec3::Y),
            camera_3d: Camera3d {
                clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(Color::rgb(0.1, 0.1, 0.1)),
                ..default()
            },
            ..default()
        },
        PanOrbitCamera::default(),
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 50000.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, 0.2, -1.0, 0.0)),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::rgb(1.0, 1.0, 1.0),
        brightness: 0.5,
    })
}