use bevy::prelude::*;
use bevy_inspector_egui::quick::{FilterQueryInspectorPlugin, ResourceInspectorPlugin};
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
        .add_plugins(ResourceInspectorPlugin::<RigidBodySimulationSettings>::default())
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
            camera_3d: Camera3d {
                clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(Color::rgb(0.1, 0.1, 0.1)),
                ..default()
            },
            ..default()
        },
        PanOrbitCamera::default(),
    ));


    for x in 0..1 {
        for z in 0..1 {
            for y in 0..2 {
                commands.spawn(RigidBodyObject {
                    state: RigidBodyState {
                        position: Vec3::new(x as f32 * 1.1, 1.0 + y as f32 * 2.0, z as f32 * 1.1),
                        orientation: Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, 0.0),
                        ..default()
                    },
                    impulses: RigidBodyImpulseAccumulator {
                        force: Vec3::ZERO,
                        torque: Vec3::new(2.0, 0.0, 0.0),
                    },
                    properties: RigidBodyProperties {
                        scale: Vec3::new(1.0, 1.0, 1.0),
                        collision_point_density: UVec3::new(4, 4, 4),
                        hardness: 1.0,
                        roughness: 1.0,
                        resilience: 0.2,
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
            }
        }
    }

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
            material: materials.add(StandardMaterial::from(Color::rgb(0.1, 0.1, 0.1))),
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