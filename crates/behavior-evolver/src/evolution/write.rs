use std::{fs, path::PathBuf};

use bevy::prelude::*;
use creature_builder::builder::node::CreatureMorphologyGraph;

use super::{
    fitness::EvolutionFitnessEval,
    generation::{EvolutionGeneration, GenerationTestingConfig},
    populate::GenerationPopulator,
    state::EvolutionState,
};
use crate::evolution::populate::CreaturePopulateFlag;


struct TrainingPaths {
    session: PathBuf,
    creatures: PathBuf,
}


fn train_path(session: &str) -> TrainingPaths {
    let get_dir = |target: &PathBuf| -> TrainingPaths {
        let sess = target.join(session);
        TrainingPaths { creatures: sess.join("creatures/"), session: sess }
    };
    let create_dir = |target: &PathBuf| {
        let paths = get_dir(target);
        fs::create_dir_all(target).expect("Unable to create training data directory");
        if !paths.session.exists() {
            fs::create_dir(paths.session).expect("Unable to create session directory");
            fs::create_dir(paths.creatures).expect("Unable to create creature directory");
        };
    };

    let home = homedir::get_my_home().unwrap().unwrap();
    if cfg!(unix) {
        let target = home.join(".local/share/evolved-creatures/training/");
        if !target.exists() {
            println!("INFO: Unix-like system detected, putting training data in ~/.local/share/evolved-creatures/");
        }
        create_dir(&target);
        get_dir(&target)
    } else {
        let target = home.join(".evolved-creatures/training/");
        if !target.exists() {
            println!("INFO: Windows-like system detected, putting training data in ~/.evolved-creatures/");
        }
        create_dir(&target);
        get_dir(&target)
    }
}

pub fn load_creature(session: &str, id: usize) -> CreatureMorphologyGraph {
    let path = train_path(session).creatures.join(format!("id-{}.ron", id));
    let creature_data = fs::read_to_string(path).expect("Unable to read existing creature data file");
    let creature_de: CreatureMorphologyGraph = ron::de::from_str(&creature_data).expect("Unable to parse creature data");
    creature_de
}

pub fn load_generation(session: &str) -> Vec<CreatureMorphologyGraph> {
    let train_dir = train_path(session);
    let gen_data = fs::read_to_string(train_dir.session.join("last-gen.dat")).expect("Unable to read existing generation file");
    let mut population = Vec::new();

    for line in gen_data[..gen_data.len() - 1].split('\n').skip(2) {
        let mut elements = line.split("  ");
        let mut grab_value = |offset: usize| {
            let text = &elements.next().expect("Invalid generation file")[offset..];
            &text[..text.len() - 1]
        };

        let id: usize = grab_value(5).parse().expect("Failed to parse id in generation file");
        let creature_data =
            fs::read_to_string(train_dir.creatures.join(format!("id-{}.ron", id))).expect("Unable to read existing creature data file");
        let creature_de: CreatureMorphologyGraph = ron::de::from_str(&creature_data).expect("Unable to parse creature data");

        population.push(creature_de);
    }

    population
}

pub fn grab_best_creature(session: &str) -> Option<usize> {
    let train_dir = train_path(session);
    let session_data = train_dir.session.join("session.dat");
    if session_data.exists() {
        let data = fs::read_to_string(session_data).expect("Failed to read existing session.dat file");

        let start = data.find("best_creature").expect("Invalid session.dat file") + 17;
        let length = data[start..].find(']').expect("Invalid session.dat file");
        return Some(data[start..start + length].parse::<usize>().expect("Invalid session.dat file"));
    }
    None
}

fn remove_dir_contents<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        fs::remove_file(entry?.path())?;
    }
    Ok(())
}


pub fn write_generation<F: EvolutionFitnessEval + Send + Sync + Default + 'static>(
    generation: Res<EvolutionGeneration<F>>,
    populator: Res<GenerationPopulator>,
    gen_test_conf: Res<GenerationTestingConfig>,
    mut next_state: ResMut<NextState<EvolutionState>>,
) {
    let train_dir = train_path(&gen_test_conf.session);
    let cur_gen = generation.current_generation;

    remove_dir_contents(&train_dir.creatures).expect("Unable to remove old creatures");
    for creature in generation.population.iter() {
        let creature_file = train_dir.creatures.join(format!("id-{}.ron", creature.creature.0));
        let serialized = ron::ser::to_string_pretty(&creature, ron::ser::PrettyConfig::default()).unwrap();
        if !creature_file.exists() {
            fs::write(creature_file, serialized).expect("Failed to write creature file");
        }
    }

    let gen_file = train_dir.session.join("last-gen.dat");
    let mut gen = String::new();

    gen.push_str(&format!("--- Generation {} ---\n\n", cur_gen));

    for ((creature, fitness), flags) in generation.population.iter().zip(generation.fitnesses.iter()).zip(generation.populate_flags.iter())
    {
        let flag_ser = ron::ser::to_string(&flags).unwrap();
        gen.push_str(&format!("id: [{}]  fitness: [{}]  flags: [{}]\n", creature.creature.0, fitness, flag_ser));
    }
    fs::write(gen_file, gen).expect("Failed to write generation file");

    let session_data = train_dir.session.join("session.dat");
    if session_data.exists() {
        let mut data = String::new();
        data.push_str(&format!(
            "--- Session data file ---\n\nname = [{}]\ncurrent_generation = [{}]\ncurrent_id = [{}]\nbest_fitness = [{}]\nbest_creature = \
             [{}]",
            &gen_test_conf.session, cur_gen, populator.current_id, populator.best_fitness, populator.best_creature,
        ));
        fs::write(session_data, data).expect("Failed to write session data file");
    } else {
        let mut data = String::new();
        data.push_str(&format!(
            "--- Session data file ---\n\nname = [{}]\ncurrent_generation = [-1]\ncurrent_id = [-1]\nbest_fitness = \
             [-1000000000000.0]\nbest_creature = [0]",
            &gen_test_conf.session
        ));
        fs::write(session_data, data).expect("Failed to write session data file");
    }

    next_state.set(EvolutionState::PopulatingGeneration);
}


pub fn load_session<F: EvolutionFitnessEval + Send + Sync + Default + 'static>(
    generation: &mut EvolutionGeneration<F>,
    populator: &mut GenerationPopulator,
    gen_test_conf: &GenerationTestingConfig,
) {
    let train_dir = train_path(&gen_test_conf.session);
    let session_data = train_dir.session.join("session.dat");

    generation.population.clear();
    generation.fitnesses.clear();
    generation.populate_flags.clear();

    if session_data.exists() {
        let data = fs::read_to_string(session_data).expect("Failed to read existing session.dat file");

        fn grab_session_data<T: std::str::FromStr>(data: &str, text: &str, offset: usize) -> T
        where
            <T as std::str::FromStr>::Err: std::fmt::Debug,
        {
            let start = data.find(text).expect("Invalid session.dat file") + offset;
            let length = data[start..].find(']').expect("Invalid session.dat file");
            data[start..start + length].parse::<T>().expect("Invalid session.dat file")
        }

        let cur_gen_data: isize = grab_session_data(&data, "current_generation", 22);
        let cur_id_data: isize = grab_session_data(&data, "current_id", 14);
        let best_fitness: f32 = grab_session_data(&data, "best_fitness", 16);
        let best_creature: usize = grab_session_data(&data, "best_creature", 17);

        if cur_gen_data < 0 {
            return;
        };
        generation.current_generation = cur_gen_data as usize;
        populator.current_id = cur_id_data as usize;
        populator.best_fitness = best_fitness;
        populator.best_creature = best_creature;

        let gen_data = fs::read_to_string(train_dir.session.join("last-gen.dat")).expect("Unable to read existing generation file");

        for line in gen_data[..gen_data.len() - 1].split('\n').skip(2) {
            let mut elements = line.split("  ");
            let mut grab_value = |offset: usize| {
                let text = &elements.next().expect("Invalid generation file")[offset..];
                &text[..text.len() - 1]
            };

            let id: usize = grab_value(5).parse().expect("Failed to parse id in generation file");
            let fitness: f32 = grab_value(10).parse().expect("Failed to parse fitness in generation file");
            let flags: CreaturePopulateFlag = ron::de::from_str(grab_value(8)).expect("Failed to parse flag in generation file");

            let creature_data =
                fs::read_to_string(train_dir.creatures.join(format!("id-{}.ron", id))).expect("Unable to read existing creature data file");
            let creature_de: CreatureMorphologyGraph = ron::de::from_str(&creature_data).expect("Unable to parse creature data");

            generation.population.push(creature_de);
            generation.fitnesses.push(fitness);
            generation.populate_flags.push(flags);
        }
    }
}
