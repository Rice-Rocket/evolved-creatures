use bevy::prelude::*;
use bevy_editor_pls::{prelude::*, controls, editor::{Editor, EditorEvent, EditorInternalState}, default_windows, egui_dock};
use rigid_body_engine_3d::sim::RigidBodySimulationSettings;


pub struct CustomEditorPlugin;

impl Plugin for CustomEditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EditorPlugin::new())
            .insert_resource(editor_controls())
            .add_systems(Update, run_editor)
            .add_systems(Startup, setup_editor);
    }
}


fn editor_controls() -> controls::EditorControls {
    let mut editor_controls = controls::EditorControls::default_bindings();

    editor_controls.unbind(controls::Action::PlayPauseEditor);
    editor_controls.insert(
        controls::Action::PlayPauseEditor,
        controls::Binding {
            input: controls::UserInput::Single(controls::Button::Keyboard(KeyCode::Escape)),
            conditions: vec![controls::BindingCondition::ListeningForText(false)],
        },
    );

    editor_controls
}

fn setup_editor(
    mut commands: Commands,
) {
    let mut internal_state = EditorInternalState::default();

    let [game, _inspector] =
        internal_state.split_right::<default_windows::inspector::InspectorWindow>(egui_dock::NodeIndex::root(), 0.75);
    let [game, _hierarchy] = internal_state.split_left::<default_windows::hierarchy::HierarchyWindow>(game, 0.2);
    let [_game, _bottom] = internal_state.split_many(
        game,
        0.8,
        egui_dock::Split::Below,
        &[
            std::any::TypeId::of::<default_windows::debug_settings::DebugSettingsWindow>(),
            std::any::TypeId::of::<default_windows::diagnostics::DiagnosticsWindow>(),
        ],
    );

    commands.insert_resource(internal_state);
}

fn run_editor(
    mut editor_events: EventReader<EditorEvent>,
    mut sim_settings: ResMut<RigidBodySimulationSettings>,
    editor: Res<Editor>,
) {
    for ev in editor_events.read() {
        if let EditorEvent::Toggle { now_active: true } = ev {
            sim_settings.pause_for(1.0);
        }
    }

    if let Some(default_window) = editor.window_state::<bevy_editor_pls::default_windows::debug_settings::DebugSettingsWindow>() {
        if default_window.pause_time {
            sim_settings.pause();
        } else {
            sim_settings.play();
        }
    }
}