pub mod fitness;
pub mod generation;
pub mod populate;
pub mod state;
pub mod write;

use std::marker::PhantomData;

use bevy::prelude::*;
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
    generation::{test_generation, test_generation_nowindow, EvolutionGeneration, GenerationTestingConfig},
    populate::{populate_generation, GenerationPopulator},
    state::{begin_training_session, EvolutionState, EvolutionTrainingEvent},
    write::write_generation,
};

pub struct CreatureEvolutionPlugin<F: EvolutionFitnessEval + Send + Sync + Default + 'static> {
    pub window: bool,
    _p: PhantomData<F>,
}

impl<F: EvolutionFitnessEval + Send + Sync + Default + 'static> Default for CreatureEvolutionPlugin<F> {
    fn default() -> Self {
        Self { window: false, _p: PhantomData }
    }
}

impl<F: EvolutionFitnessEval + Send + Sync + Default + 'static> CreatureEvolutionPlugin<F> {
    pub fn new(window: bool) -> Self {
        Self { window, _p: PhantomData }
    }
}

impl<F: EvolutionFitnessEval + Send + Sync + Default + 'static> Plugin for CreatureEvolutionPlugin<F> {
    fn build(&self, app: &mut App) {
        app.add_plugins(CreatureEnvironmentPlugin { window: self.window })
            .add_state::<EvolutionState>()
            .init_resource::<EvolutionGeneration<F>>()
            .init_resource::<GenerationPopulator>()
            .init_resource::<GenerationTestingConfig>()
            .add_event::<EvolutionTrainingEvent>()
            .add_systems(OnEnter(EvolutionState::BeginTrainingSession), begin_training_session::<F>)
            .add_systems(OnEnter(EvolutionState::WritingGeneration), write_generation::<F>)
            .add_systems(OnEnter(EvolutionState::PopulatingGeneration), populate_generation::<F>);

        if self.window {
            app.add_systems(Update, test_generation::<F>);
        } else {
            app.add_systems(Update, test_generation_nowindow::<F>);
        }
    }
}


pub struct CreatureEnvironmentPlugin {
    pub window: bool,
}

impl Plugin for CreatureEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CreatureBuilderPlugin)
            .insert_resource(CreatureBuilderConfig::default())
            .add_plugins(RapierPhysicsPlugin::<ContactFilter>::default())
            .insert_resource(RapierConfiguration { timestep_mode: TimestepMode::Fixed { dt: 1.0 / 60.0, substeps: 4 }, ..default() });

        if self.window {
            app.add_plugins(PanOrbitCameraPlugin)
                .add_plugins(ScreenDiagnosticsPlugin::default())
                .add_systems(Startup, (setup, setup_ground))
                .add_plugins(ScreenFrameDiagnosticsPlugin);
        } else {
            app.add_systems(Startup, setup_ground_nowindow);
        }
    }
}


#[derive(Component)]
pub struct GroundMarker;


fn setup_ground_nowindow(mut commands: Commands) {
    commands.spawn((
        RigidBody::KinematicPositionBased,
        Velocity { linvel: Vec3::ZERO, angvel: Vec3::ZERO },
        GravityScale(1.0),
        ActiveEvents::COLLISION_EVENTS,
        ActiveHooks::FILTER_CONTACT_PAIRS,
        ContactFilterTag::GroundGroup,
        Collider::cuboid(500.0, 5.0, 500.0),
        Friction { coefficient: 0.75, combine_rule: CoefficientCombineRule::Average },
        Restitution { coefficient: 0.0, combine_rule: CoefficientCombineRule::Average },
        ColliderMassProperties::Density(1.0),
        Transform::from_xyz(0.0, -5.0, 0.0),
        GlobalTransform::default(),
        GroundMarker,
        Name::new("Ground"),
    ));
}


fn setup_ground(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn((
        RigidBody::KinematicPositionBased,
        Velocity { linvel: Vec3::ZERO, angvel: Vec3::ZERO },
        GravityScale(1.0),
        ActiveEvents::COLLISION_EVENTS,
        ActiveHooks::FILTER_CONTACT_PAIRS,
        ContactFilterTag::GroundGroup,
        Collider::cuboid(500.0, 5.0, 500.0),
        Friction { coefficient: 0.75, combine_rule: CoefficientCombineRule::Average },
        Restitution { coefficient: 0.0, combine_rule: CoefficientCombineRule::Average },
        ColliderMassProperties::Density(1.0),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1000.0, 10.0, 1000.0))),
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
        GroundMarker,
        Name::new("Ground"),
    ));
}


fn setup(mut commands: Commands) {
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
