pub mod fitness;
pub mod generation;
pub mod populate;
pub mod state;

use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_editor_pls::editor_controls::rapier::RapierPhysicsEditorPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_rapier3d::{
    dynamics::{CoefficientCombineRule, GravityScale, RigidBody, Velocity},
    geometry::{ActiveEvents, ActiveHooks, Collider, ColliderMassProperties, Friction, Restitution},
    plugin::{RapierConfiguration, RapierPhysicsPlugin, TimestepMode},
};
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use creature_builder::{
    config::CreatureBuilderConfig,
    sensor::{ContactFilter, ContactFilterTag},
    CreatureBuilderPlugin,
};

use self::{
    fitness::EvolutionFitnessEval,
    generation::{test_generation, EvolutionGeneration, GenerationTestingConfig},
    populate::{populate_generation, GenerationPopulator},
    state::EvolutionState,
};

#[derive(Default)]
pub struct CreatureEvolutionPlugin<F: EvolutionFitnessEval + Send + Sync + Default + 'static> {
    _p: PhantomData<F>,
}

impl<F: EvolutionFitnessEval + Send + Sync + Default + 'static> Plugin for CreatureEvolutionPlugin<F> {
    fn build(&self, app: &mut App) {
        app.add_plugins(CreatureEnvironmentPlugin)
            .add_state::<EvolutionState>()
            .init_resource::<EvolutionGeneration<F>>()
            .init_resource::<GenerationPopulator>()
            .init_resource::<GenerationTestingConfig>()
            .add_systems(Update, test_generation::<F>)
            .add_systems(OnEnter(EvolutionState::PopulatingGeneration), populate_generation::<F>);
    }
}


pub struct CreatureEnvironmentPlugin;

impl Plugin for CreatureEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CreatureBuilderPlugin)
            .insert_resource(CreatureBuilderConfig::default())
            .add_systems(Startup, (setup, setup_ground))
            .add_plugins(RapierPhysicsPlugin::<ContactFilter>::default())
            .add_plugins(PanOrbitCameraPlugin)
            .add_plugins(ScreenDiagnosticsPlugin::default())
            .add_plugins(ScreenFrameDiagnosticsPlugin)
            .add_plugins(RapierPhysicsEditorPlugin)
            .insert_resource(RapierConfiguration {
                timestep_mode: TimestepMode::Variable { max_dt: 1.0 / 60.0, time_scale: 0.75, substeps: 4 },
                ..default()
            });
    }
}


fn setup_ground(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
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
        Name::new("Ground"),
    ));
}


fn setup(mut gizmo_config: ResMut<GizmoConfig>, mut commands: Commands) {
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
        directional_light: DirectionalLight { color: Color::WHITE, illuminance: 50000.0, shadows_enabled: false, ..default() },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, 0.2, -1.0, 0.0)),
        ..default()
    });

    commands.insert_resource(AmbientLight { color: Color::rgb(1.0, 1.0, 1.0), brightness: 0.5 })
}
