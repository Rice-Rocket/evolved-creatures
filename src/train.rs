use behavior_evolver::{
    evolution::{
        fitness::{jump::JumpFitnessEval, walk::WalkFitnessEval},
        generation::GenerationTestingConfig,
        populate::GenerationPopulator,
        state::{EvolutionState, EvolutionTrainingEvent},
        CreatureEvolutionPlugin,
    },
    mutate::{MutateMorphologyParams, RandomMorphologyParams},
};
use bevy::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use termion::{color, style};

#[derive(Resource)]
pub struct TrainConfig {
    pub session: String,
    pub visual: bool,
    pub silent: bool,
    pub test_time: usize,
    pub elitism: f32,
    pub rand_percent: f32,
    pub pop_size: usize,
    pub num_mutations: usize,
    pub fitness_fn: String,
}

impl Default for TrainConfig {
    fn default() -> Self {
        Self {
            session: String::from("default-session"),
            visual: false,
            silent: false,
            test_time: 180,
            elitism: 0.25,
            rand_percent: 0.03,
            pop_size: 250,
            num_mutations: 20,
            fitness_fn: String::from("jump"),
        }
    }
}

pub fn train(conf: TrainConfig) {
    let mut app = App::new();
    app.add_systems(Startup, setup);

    if conf.visual {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window { present_mode: bevy::window::PresentMode::AutoNoVsync, ..default() }),
            ..default()
        }));
    } else {
        app.add_plugins(MinimalPlugins).add_plugins(bevy::transform::TransformPlugin).add_plugins(bevy::hierarchy::HierarchyPlugin);
    }

    if conf.fitness_fn == "jump" {
        app.add_plugins(CreatureEvolutionPlugin::<JumpFitnessEval>::new(conf.visual));
    } else if conf.fitness_fn == "walk" {
        app.add_plugins(CreatureEvolutionPlugin::<WalkFitnessEval>::new(conf.visual));
    } else {
        panic!("Invalid fitness function");
    }

    if !conf.silent {
        app.insert_resource(GenerationProgressBar(ProgressBar::new(0), false)).add_systems(Update, print_info);
    }

    app.insert_resource(conf);
    app.run();
}


#[derive(Resource)]
struct GenerationProgressBar(ProgressBar, bool);

fn print_info(
    mut train_evr: EventReader<EvolutionTrainingEvent>,
    mut bar: ResMut<GenerationProgressBar>,
    conf: Res<TrainConfig>,
    populator: Res<GenerationPopulator>,
) {
    for ev in train_evr.read() {
        match ev {
            EvolutionTrainingEvent::FinishedTestingCreature => bar.0.inc(1),
            EvolutionTrainingEvent::StartTestingGeneration(gen) => {
                if bar.1 {
                    bar.0.finish_and_clear();
                }
                bar.1 = true;

                let mut template = format!("{}{}--- Generation {} ---{}\n", style::Bold, color::Fg(color::LightCyan), gen, style::Reset);
                template.push_str(&format!("--- <{}Ctrl-c{}> to exit ---\n", color::Fg(color::Red), color::Fg(color::Reset)));
                template.push_str(&format!(
                    "--- Best fitness: {}{:.3}{} - Best creature: id({}{}{}) ---\n",
                    color::Fg(color::Yellow),
                    populator.best_fitness,
                    color::Fg(color::Reset),
                    color::Fg(color::Yellow),
                    populator.best_creature,
                    color::Fg(color::Reset),
                ));
                template.push_str("{spinner:.green} [{elapsed}] [{bar:50.white/blue}] {pos}/{len} ({eta})");

                bar.0 = ProgressBar::new(conf.pop_size as u64)
                    .with_style(ProgressStyle::with_template(&template).unwrap().progress_chars("=> ").tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"));
            },
        }
    }
}

fn setup(mut commands: Commands, mut state: ResMut<NextState<EvolutionState>>, conf: Res<TrainConfig>) {
    commands.insert_resource(GenerationTestingConfig {
        test_time: conf.test_time,
        session: conf.session.clone(),
        wait_for_fall: true,
        ..Default::default()
    });
    commands.insert_resource(GenerationPopulator::new(
        conf.elitism,
        conf.rand_percent,
        conf.pop_size,
        MutateMorphologyParams::default(),
        RandomMorphologyParams::default(),
    ));
    state.set(EvolutionState::BeginTrainingSession);
}
