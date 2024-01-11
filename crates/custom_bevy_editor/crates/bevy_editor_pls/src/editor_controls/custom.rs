use bevy::prelude::*;
use crate::{prelude::*, controls, editor::{EditorEvent, EditorInternalState}, default_windows, egui_dock, editor_window::EditorWindow};
use rigid_body_engine_3d::sim::RigidBodySimulationSettings;


pub struct CustomPhysicsEditorPlugin;

impl Plugin for CustomPhysicsEditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EditorPlugin::new())
            .insert_resource(editor_controls())
            .add_systems(Update, run_editor)
            .add_systems(Startup, setup_editor)
            .add_editor_window::<PhysicsSettingsWindow>();
    }
}


struct PhysicsSettingsWindow;

#[derive(Default)]
struct PhysicsSettingsWindowState {
    pub pause_time: bool,
}

impl EditorWindow for PhysicsSettingsWindow {
    type State = PhysicsSettingsWindowState;
    const NAME: &'static str = "Physics settings";

    fn ui(world: &mut World, mut cx: crate::editor_window::EditorWindowContext, ui: &mut bevy_inspector_egui::egui::Ui) {
        let state = cx.state_mut::<PhysicsSettingsWindow>().unwrap();

        let available_size = ui.available_size();
        let horizontal = available_size.x > available_size.y;

        horizontal_if(ui, horizontal, |ui| {
            debug_ui_options(world, state, ui);
        });
    }
}

pub fn horizontal_if<R>(
    ui: &mut egui::Ui,
    horizontal: bool,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    if horizontal {
        ui.horizontal(add_contents).inner
    } else {
        add_contents(ui)
    }
}

fn debug_ui_options(
    world: &mut World,
    state: &mut PhysicsSettingsWindowState,
    ui: &mut egui::Ui,
) {
    egui::Grid::new("physics settings").show(ui, |ui| {
        let keys = world.resource::<Input<KeyCode>>().clone();
        let mut sim_settings = world.resource_mut::<RigidBodySimulationSettings>();

        ui.label("Pause");
        let mut changed = false;
        if keys.just_pressed(KeyCode::P) {
            state.pause_time = !state.pause_time;
            changed = true;
        }
        if ui.checkbox(&mut state.pause_time, "").changed() || changed {
            if state.pause_time {
                sim_settings.pause();
            } else {
                sim_settings.play();
            }
        }
        ui.end_row();

        ui.label("Speed Modifier");
        let mut speed = sim_settings.speed;
        if ui
            .add(
                egui::DragValue::new(&mut speed)
                    .clamp_range(0f32..=2f32)
                    .speed(0.025),
            ).changed()
        {
            sim_settings.speed = speed;
        }
        ui.end_row();

        ui.label("Step Size");
        let mut step_rate = sim_settings.step_dt * 60.0;
        if ui
            .add(
                egui::DragValue::new(&mut step_rate)
                    .clamp_range(0.1..=2f32)
                    .speed(0.025),
            ).changed()
        {
            sim_settings.step_dt = step_rate / 60.0;
        }
        ui.end_row();

        if ui.button("Step").clicked() {
            sim_settings.step();
        }
    });
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
            std::any::TypeId::of::<PhysicsSettingsWindow>(),
            std::any::TypeId::of::<default_windows::debug_settings::DebugSettingsWindow>(),
            std::any::TypeId::of::<default_windows::diagnostics::DiagnosticsWindow>(),
        ],
    );

    commands.insert_resource(internal_state);
}

fn run_editor(
    mut editor_events: EventReader<EditorEvent>,
    mut sim_settings: ResMut<RigidBodySimulationSettings>,
) {
    for ev in editor_events.read() {
        if let EditorEvent::Toggle { now_active: true } = ev {
            sim_settings.pause_for(1.0);
        }
    }
}