use bevy::prelude::*;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use bevy_panorbit_camera::*;


use rigid_body_engine_3d::prelude::*;

use crate::editor::CustomEditorPlugin;


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
        // .add_systems(Update, update)
        
        .add_plugins(RigidBodySimulationPlugin)

        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(CustomEditorPlugin)

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

    commands.spawn((RigidBodyObject {
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
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.1, 0.1, 0.1),
                perceptual_roughness: 0.9,
                reflectance: 0.1,
                metallic: 0.0,
                ..default()
            }),
            ..default()
        },
        ..default()
    }, Name::new("Ground")));

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

#[allow(dead_code)]
fn joint_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let rb1 = commands.spawn((RigidBodyObject {
        state: RigidBodyState {
            position: Vec3::new(0.0, 2.0, 0.0),
            orientation: Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, 0.0),
            ..default()
        },
        impulses: RigidBodyImpulseAccumulator {
            force: Vec3::ZERO,
            torque: Vec3::ZERO,
            // torque: Vec3::new(2.0, 2.0, -2.0),
        },
        properties: RigidBodyProperties {
            scale: Vec3::new(1.0, 1.0, 1.0),
            collision_point_density: UVec3::new(4, 4, 4),
            hardness: 1.0,
            roughness: 1.0,
            resilience: 0.5,
            mass: 1.0,
            is_collider: false,
            ..default()
        },
        object: PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.5, 0.0),
                perceptual_roughness: 0.9,
                reflectance: 0.1,
                metallic: 0.0,
                ..default()
            }),
            ..default()
        },
        ..default()
    }, Name::new("Orange RB"))).id();

    let rb2 = commands.spawn((RigidBodyObject {
        state: RigidBodyState {
            position: Vec3::new(0.0, 3.0, 0.0),
            orientation: Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, 0.0),
            ..default()
        },
        impulses: RigidBodyImpulseAccumulator {
            force: Vec3::ZERO,
            torque: Vec3::new(0.0, 0.0, 0.0),
        },
        properties: RigidBodyProperties {
            scale: Vec3::new(1.0, 1.0, 1.0),
            collision_point_density: UVec3::new(4, 4, 4),
            hardness: 1.0,
            roughness: 1.0,
            resilience: 0.5,
            mass: 1.0,
            is_collider: false,
            ..default()
        },
        object: PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(1.0, 0.0, 0.0),
                perceptual_roughness: 0.9,
                reflectance: 0.1,
                metallic: 0.0,
                ..default()
            }),
            ..default()
        },
        ..default()
    }, Name::new("Red RB"))).id();
    
    commands.spawn((RBJoint {
        ty: RBSphericalJoint,
        props: RBJointProperties {
            body_1: rb1,
            body_2: rb2,
            position_1: Vec3::new(0.0, 1.0, 0.0),
            position_2: Vec3::new(0.0, -1.0, 0.0),
            tangent: Vec3::new(1.0, 0.0, 0.0),
            bitangent: Vec3::new(0.0, 0.0, 1.0),
            joint_limits: Vec2::new(0.0, 1.0),
            // joint_limits: Vec2::new(0.3, 1.0),
            ..default()
        },
        ..default()
    }, Name::new("Joint")));
}

#[allow(dead_code)]
fn tower_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let tower_height = 80;
    // let tower_width = 1;
    let tower_height = 9;
    let tower_width = 3;
    for x_pos in 0..tower_width {
        for z_pos in 0..tower_width {
            for y in 0..tower_height {
                let uvw = Vec3::new(x_pos as f32 / tower_width as f32, y as f32 / tower_height as f32, z_pos as f32 / tower_width as f32);
                let x = x_pos - tower_width / 2;
                let z = z_pos - tower_width / 2;
                commands.spawn(RigidBodyObject {
                    state: RigidBodyState {
                        position: Vec3::new(x as f32 * 1.1, 1.0 + y as f32 * 2.0, z as f32 * 1.1),
                        orientation: Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, 0.0),
                        ..default()
                    },
                    impulses: RigidBodyImpulseAccumulator {
                        force: Vec3::ZERO,
                        torque: Vec3::new(2.0, 2.0, -2.0),
                    },
                    properties: RigidBodyProperties {
                        scale: Vec3::new(1.0, 1.0, 1.0),
                        collision_point_density: UVec3::new(4, 4, 4),
                        hardness: 1.0,
                        roughness: 1.0,
                        resilience: 0.5,
                        mass: 1.0,
                        ..default()
                    },
                    object: PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
                        material: materials.add(StandardMaterial {
                            base_color: Color::rgb(1.0, uvw.y.sin(), 0.0),
                            perceptual_roughness: 0.9,
                            reflectance: 0.1,
                            metallic: 0.0,
                            ..default()
                        }),
                        ..default()
                    },
                    ..default()
                });
            }
        }
    }
}