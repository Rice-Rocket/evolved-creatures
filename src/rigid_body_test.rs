use bevy::prelude::*;
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use bevy_panorbit_camera::*;


use rigid_body_engine_3d::prelude::*;


pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        // .add_systems(Update, update)
        
        .add_plugins(RigidBodySimulationPlugin)

        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        // .add_plugins(ResourceInspectorPlugin::<SoftBodySimulationSettings>::default())
        .add_plugins(FilterQueryInspectorPlugin::<With<RigidBodyProperties>>::default())

        .run();
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
            ..default()
        },
        PanOrbitCamera::default(),
    ));

    commands.spawn(RigidBodyObject {
        state: RigidBodyState {
            position: Vec3::new(0.0, 4.0, 0.0),
            orientation: Quat::from_euler(EulerRot::YXZ, 0.0, 1.0, 0.0),
            angular_momentum: Vec3::new(0.0, 1.0, 0.0),
            ..default()
        },
        properties: RigidBodyProperties {
            scale: Vec3::new(1.0, 5.0, 1.0),
            mass: 1.0,
            ..default()
        },
        object: PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
            material: materials.add(StandardMaterial::from(Color::rgb(1.0, 0.0, 0.0))),
            ..default()
        },
        ..default()
    });

    commands.spawn(RigidBodyObject {
        state: RigidBodyState {
            position: Vec3::new(0.0, -5.0, 0.0),
            ..default()
        },
        properties: RigidBodyProperties {
            scale: Vec3::new(100.0, 10.0, 100.0),
            hardness: 1.0,
            roughness: 1.0,
            resilience: 0.2,
            mass: 1.0,
            locked: true,
            ..default()
        },
        object: PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
            material: materials.add(StandardMaterial::from(Color::rgb(1.0, 1.0, 1.0))),
            ..default()
        },
        ..default()
    });

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
}


// fn update(

// ) {

// }