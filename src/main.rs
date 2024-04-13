use std::{env, fs, process::Command, time::Duration};

use behavior_evolver::evolution::write;
use playback::{PlaybackConfig, PlaybackMode};
use train::TrainConfig;

mod playback;
mod train;


pub struct InvalidUsageError(pub String);

fn err(s: &str) -> Result<(), InvalidUsageError> {
    Err(InvalidUsageError(s.to_owned()))
}

fn expect<T>(v: Option<T>, s: &str) -> Result<T, InvalidUsageError> {
    match v {
        Some(x) => Ok(x),
        None => Err(InvalidUsageError(s.to_owned())),
    }
}

fn expect_res<T, E>(v: Result<T, E>, s: &str) -> Result<T, InvalidUsageError> {
    match v {
        Ok(x) => Ok(x),
        Err(_) => Err(InvalidUsageError(s.to_owned())),
    }
}

fn print_help(args: &[String]) {
    println!();
    println!("USAGE:");
    println!("    {} train [session] [TRAIN OPTIONS]", args[0]);
    println!("            Begin a new or attach to an existing training session");
    println!();
    println!("    {} play [session] [-c|-g] [PLAYBACK OPTIONS]", args[0]);
    println!("            Playback a creature or entire generation");
    println!();
    println!("    {} plist [playlist|-l] [PLAYLIST OPTIONS]", args[0]);
    println!("            Perform operations on a list of selected creatures");
    println!();
    println!("    {} session [name|-l] [SESSION OPTIONS]", args[0]);
    println!("            Perform operations on training sessions");
    println!();
    println!("    {} help", args[0]);
    println!("            Display this message");
    println!();
    println!("TRAIN OPTIONS:");
    println!("    -v, --visual");
    println!("            Open a window showing the training in progress");
    println!();
    println!("    -s, --silent");
    println!("            Don't print any progress updates");
    println!();
    println!("    -o, --overwrite");
    println!("            Overwrite the session if it already exists instead of attaching to it");
    println!();
    println!("    -t, --test-time <TEST_TIME>");
    println!("            Number of simulation steps that each creatures should be tested for");
    println!("            Should be >0");
    println!("            Default: 180");
    println!();
    println!("    -p, --population <POPULATION>");
    println!("            The number of creatures in each generation");
    println!("            Should be >0");
    println!("            Default: 250");
    println!();
    println!("    -n, --num-mutations <NUM_MUTATIONS>");
    println!("            The maximum number of mutations each creature will sustain");
    println!("            Default: 80");
    println!();
    println!("    -e, --elitism <ELITISM>");
    println!("            The portion of the previous generation's population that is preserved");
    println!("            and mutated to make up the next generation");
    println!("            Should be in interval [0..1]");
    println!("            Default: 0.25");
    println!();
    println!("    -r, --rand-percent <RAND_PERCENT>");
    println!("            The portion of the next generation that will be made up of completely");
    println!("            random creatures");
    println!("            Should be in interval [0..1]");
    println!("            Default: 0.03");
    println!();
    println!("    -f, --fitness <FITNESS_FN>");
    println!("            The fitness function to use when evaluating creatures");
    println!("            Options: [jump, walk]");
    println!("            Default: jump");
    println!();
    println!("PLAYBACK OPTIONS:");
    println!("    -c, --creature <CREATURE_ID>");
    println!("            Playback a specific creature");
    println!();
    println!("    -g, --generation <GENERATION_ID>");
    println!("            Playback a specific generation");
    println!();
    println!("    -b, --best");
    println!("            Playback the best creature");
    println!();
    println!("    -a, --auto-cycle <CYCLE_DELAY>");
    println!("            Enable auto-cycling through creatures with specified delay");
    println!("            Default: unset; no auto-cycle");
    println!();
    println!("PLAYLIST OPTIONS:");
    println!("    -n, --new [<SESSION> <CREATURE_ID>]+");
    println!("            Create a new playlist with the given creatures");
    println!();
    println!("    -p, --play");
    println!("            Playback an existing playlist");
    println!();
    println!("    -l, --list");
    println!("            List all playlists");
    println!();
    println!("SESSION OPTIONS:");
    println!("    -d, --delete");
    println!("            Delete the session");
    println!();
    println!("    -l, --list");
    println!("            List all sessions");
}

fn parse_args(args: Vec<String>) -> Result<(), InvalidUsageError> {
    if args[1] == "train" {
        let mut train_config = TrainConfig { session: expect(args.get(2), "Expected [session]")?.clone(), ..Default::default() };

        if args.len() > 2 {
            let mut opts = args[3..].iter();

            while let Some(arg) = opts.next() {
                if arg == "-v" || arg == "--visual" {
                    train_config.visual = true;
                } else if arg == "-s" || arg == "--silent" {
                    train_config.silent = true;
                } else if arg == "-o" || arg == "--overwrite" {
                    train_config.overwrite = true;
                } else if arg == "-t" || arg == "--test-time" {
                    train_config.test_time =
                        expect_res(expect(opts.next(), "Expected <TEST_TIME>")?.parse::<usize>(), "Invalid <TEST_TIME>")?;
                } else if arg == "-p" || arg == "--population" {
                    train_config.pop_size =
                        expect_res(expect(opts.next(), "Expected <POPULATION>")?.parse::<usize>(), "Invalid <POPULATION>")?;
                } else if arg == "-n" || arg == "--num-mutations" {
                    train_config.num_mutations =
                        expect_res(expect(opts.next(), "Expected <NUM_MUTATIONS>")?.parse::<usize>(), "Invalid <NUM_MUTATIONS>")?;
                } else if arg == "-e" || arg == "--elitism" {
                    train_config.elitism = expect_res(expect(opts.next(), "Expected <ELITISM>")?.parse::<f32>(), "Invalid <ELITISM>")?;
                } else if arg == "-r" || arg == "--rand_percent" {
                    train_config.rand_percent =
                        expect_res(expect(opts.next(), "Expected <RAND_PERCENT>")?.parse::<f32>(), "Invalid <RAND_PERCENT>")?;
                } else if arg == "-f" || arg == "--fitness" {
                    let fun = expect(opts.next(), "Expected <FITNESS_FN>")?;
                    if fun == "jump" || fun == "walk" {
                        train_config.fitness_fn = fun.to_string();
                    } else {
                        return err("Invalid <FITNESS_FN>");
                    }
                }
            }
        }

        println!();
        println!("Training with the following config: ");
        println!("    session = {}", train_config.session);
        println!("    visual = {}", train_config.visual);
        println!("    silent = {}", train_config.silent);
        println!("    test_time = {}", train_config.test_time);
        println!("    population = {}", train_config.pop_size);
        println!("    num_mutations = {}", train_config.num_mutations);
        println!("    elitism = {}", train_config.elitism);
        println!("    rand_percent = {}", train_config.rand_percent);
        println!("    fitness = {}", train_config.fitness_fn);
        println!();

        train::train(train_config);
    } else if args[1] == "play" {
        let mut playback_config = PlaybackConfig { session: expect(args.get(2), "Expected [session]")?.clone(), ..Default::default() };
        let mut supplied_mode = false;

        if args.len() > 2 {
            let mut opts = args[3..].iter();

            while let Some(arg) = opts.next() {
                if arg == "-c" || arg == "--creature" {
                    supplied_mode = true;
                    playback_config.mode = PlaybackMode::Creature(expect_res(
                        expect(args.get(4), "Expected <CREATURE_ID>")?.parse::<usize>(),
                        "Invalid <CREATURE_ID>",
                    )?);
                } else if arg == "-g" || arg == "--generation" {
                    supplied_mode = true;
                    playback_config.mode = PlaybackMode::Generation;
                } else if arg == "-b" || arg == "--best" {
                    supplied_mode = true;
                    playback_config.mode = PlaybackMode::BestCreature(0);
                } else if arg == "-a" || arg == "--auto-cycle" {
                    playback_config.auto_cycle = Some(Duration::from_secs_f32(expect_res(
                        expect(opts.next(), "Expected <CYCLE_DELAY>")?.parse::<f32>(),
                        "Invalid <CYCLE_DELAY>",
                    )?));
                }
            }
        }

        if !supplied_mode {
            return err("Invalid usage, expected [-c|-g|-b]");
        }

        if let PlaybackMode::BestCreature(_) = playback_config.mode {
            playback_config.mode =
                PlaybackMode::BestCreature(expect(write::grab_best_creature(&playback_config.session), "session.dat file does not exist")?);
        }


        let (mode, id) = match playback_config.mode {
            PlaybackMode::Creature(id) => ("creature", format!("{}", id)),
            PlaybackMode::Generation => ("generation", "N/A".to_string()),
            PlaybackMode::BestCreature(id) => ("best_creature", format!("{}", id)),
            PlaybackMode::List(_) => unreachable!(),
        };

        println!();
        println!("Executing playback with the following config");
        println!("    mode = {}", mode);
        println!("    id = {}", id);
        match playback_config.auto_cycle {
            Some(duration) => println!("    auto-cycle = {}", duration.as_secs_f32()),
            None => println!("    auto-cycle = false"),
        }
        println!();

        playback::play(playback_config);
    } else if args[1] == "plist" {
        let list = expect(args.get(2), "Expected [playlist|-l]")?.clone();
        if list == "-l" || list == "--list" {
            let list_dir = write::data_path().join("playlists");
            let output = if cfg!(target_os = "windows") {
                let cmd = format!("dir {}", list_dir.to_str().expect("Unable to convert playlist path to string"));
                Command::new("cmd").args(["/C", &cmd]).output().expect("Failed to list playlists")
            } else {
                let cmd = format!("ls {}", list_dir.to_str().expect("Unable to convert playlist path to string"));
                Command::new("sh").args(["-c", &cmd]).output().expect("Failed to list playlists")
            };
            println!("{}", String::from_utf8(output.stdout).expect("Invalid ls output").replace('\n', "  "));
            return Ok(());
        }
        let mut is_playing = false;
        let mut playback_config = PlaybackConfig { session: String::new(), mode: PlaybackMode::List(list.clone()), ..Default::default() };

        if args.len() > 2 {
            let mut opts = args[3..].iter();

            while let Some(arg) = opts.next() {
                if arg == "-p" || arg == "--play" {
                    is_playing = true;
                } else if arg == "-n" || arg == "--new" {
                    let mut sessions = String::new();
                    #[allow(clippy::while_let_on_iterator)]
                    while let Some(sess) = opts.next() {
                        sessions.push_str(sess);
                        sessions.push('\n');
                        let id_arg = expect(opts.next(), "Expected <CREATURE_ID>")?;
                        let id = if id_arg == "-b" || id_arg == "--best" {
                            expect(write::grab_best_creature(sess), "No best creature found")?
                        } else {
                            expect_res(id_arg.parse::<usize>(), "Invalid <CREATURE_ID>")?
                        };
                        sessions.push_str(&id.to_string());
                        sessions.push('\n');
                    }

                    if sessions.is_empty() {
                        return err("Expected at least one <SESSION> and <CREATURE_ID>");
                    }

                    let home = homedir::get_my_home().unwrap().unwrap();
                    let lists = home.join(".local/share/evolved-creatures/playlists/");
                    expect_res(fs::write(lists.join(format!("{}.plst", list)), sessions), "Unable to write playlist file")?;
                } else if arg == "-a" || arg == "--auto-cycle" {
                    playback_config.auto_cycle = Some(Duration::from_secs_f32(expect_res(
                        expect(opts.next(), "Expected <CYCLE_DELAY>")?.parse::<f32>(),
                        "Invalid <CYCLE_DELAY>",
                    )?));
                }
            }
        }

        if let PlaybackMode::BestCreature(_) = playback_config.mode {
            playback_config.mode =
                PlaybackMode::BestCreature(expect(write::grab_best_creature(&playback_config.session), "session.dat file does not exist")?);
        }


        if is_playing {
            println!();
            println!("Executing playback with the following config");
            println!("    mode = list");
            println!("    list = {}", list);
            match playback_config.auto_cycle {
                Some(duration) => println!("    auto-cycle = {}", duration.as_secs_f32()),
                None => println!("    auto-cycle = false"),
            }
            println!();

            playback::play(playback_config);
        }
    } else if args[1] == "session" {
        let name = expect(args.get(2), "Expected [session|-l]")?.clone();
        if name == "-l" || name == "--list" {
            let list_dir = write::data_path().join("training");
            let output = if cfg!(target_os = "windows") {
                let cmd = format!("dir {}", list_dir.to_str().expect("Unable to convert session path to string"));
                Command::new("cmd").args(["/C", &cmd]).output().expect("Failed to list sessions")
            } else {
                let cmd = format!("ls {}", list_dir.to_str().expect("Unable to convert session path to string"));
                Command::new("sh").args(["-c", &cmd]).output().expect("Failed to list sessions")
            };
            println!("{}", String::from_utf8(output.stdout).expect("Invalid ls output").replace('\n', "  "));
            return Ok(());
        }
        if args.len() > 2 {
            let opts = args[3..].iter();
            for arg in opts {
                if arg == "-d" || arg == "--delete" {
                    let dir = write::train_path(&name).session;
                    fs::remove_dir_all(dir).expect("Unable to delete session");
                    println!("INFO: removed session {}", name);
                    return Ok(());
                }
            }
        }
    } else {
        return err("Invalid first argument");
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 || args[1] == "help" {
        print_help(&args);
        return;
    }

    match parse_args(args) {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {}", e.0);
        },
    }
}
