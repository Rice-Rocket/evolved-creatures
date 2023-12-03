use bevy::prelude::*;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, FilterQueryInspectorPlugin};
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
        // .add_plugins(FilterQueryInspectorPlugin::<With<ResizableSoftBodyProperties>>::default())

        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        PanOrbitCamera::default(),
    ));


}


fn update(

) {

}