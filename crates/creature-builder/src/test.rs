use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use bevy_editor_pls::prelude::*;

#[path = "./lib.rs"]
pub mod creature_builder;

use bevy_rapier3d::prelude::*;
use creature_builder::{globals::CreatureBuilderGlobals, CreatureBuilderPlugin, limb::CreatureLimbBundle};


pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))

        .add_systems(Startup, (setup, joint_scene))
        .add_plugins(CreatureBuilderPlugin)

        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(RapierPhysicsEditorPlugin)

        .run();
}


fn joint_scene(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        CreatureLimbBundle::new()
            .with_color(Color::rgb(0.8, 0.1, 0.1))
            .with_groups(CreatureBuilderGlobals::MAIN_COLLISION_GROUP)
            .with_size(Vec3::splat(1.0))
            .with_transform(Transform::from_xyz(0.0, 2.0, 0.0))
            .finish(&mut meshes, &mut materials),
        Name::new("Red Body")
    ));

    commands.spawn((
        CreatureLimbBundle::new()
            .with_color(Color::rgb(0.1, 0.1, 0.8))
            .with_groups(CreatureBuilderGlobals::MAIN_COLLISION_GROUP)
            .with_size(Vec3::splat(1.0))
            .with_transform(Transform::from_xyz(0.0, 3.0, 0.0))
            .finish(&mut meshes, &mut materials),
        Name::new("Blue Body")
    ));
}


fn setup(
    mut gizmo_config: ResMut<GizmoConfig>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    gizmo_config.depth_bias = -1.0;

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(5.0, 3.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera_3d: Camera3d {
                clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(Color::rgb(0.1, 0.1, 0.1)),
                ..default()
            },
            ..default()
        },
        PanOrbitCamera::default(),
    ));

    commands.spawn((
        RigidBody::KinematicPositionBased,
        Velocity { linvel: Vec3::ZERO, angvel: Vec3::ZERO },
        GravityScale(1.0),
        ExternalForce { force: Vec3::ZERO, torque: Vec3::ZERO },

        Collider::cuboid(50.0, 5.0, 50.0),
        CreatureBuilderGlobals::GROUND_COLLISION_GROUP,
        Friction { coefficient: 0.5, combine_rule: CoefficientCombineRule::Average },
        Restitution { coefficient: 0.1, combine_rule: CoefficientCombineRule::Average },
        ColliderMassProperties::Mass(1.0),

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