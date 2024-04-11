use std::env;

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
    println!("USAGE:");
    println!("    {} train [session] [TRAIN OPTIONS]", args[0]);
    println!("    {} play [session] [PLAYBACK OPTIONS]", args[0]);
    println!("    {} help", args[0]);
    println!();
    println!("TRAIN OPTIONS:");
    println!("    -v, --visual");
    println!("            Open a window showing the training in progress");
    println!();
    println!("    -s, --silent");
    println!("            Don't print any progress updates");
    println!();
    println!("    -t, --test-time <TEST_TIME>");
    println!("            Number of simulation steps that each creatures should be tested for");
    println!("            Should be >0");
    println!();
    println!("    -p, --population <POPULATION>");
    println!("            The number of creatures in each generation");
    println!("            Should be >0");
    println!();
    println!("    -e, --elitism <ELITISM>");
    println!("            The portion of the previous generation's population that is preserved");
    println!("            and mutated to make up the next generation");
    println!("            Should be in interval [0..1]");
    println!();
    println!("    -r, --rand-percent <RAND_PERCENT>");
    println!("            The portion of the next generation that will be made up of completely");
    println!("            random creatures");
    println!("            Should be in interval [0..1]");
    println!();
    println!("    -f, --fitness <FITNESS_FN>");
    println!("            The fitness function to use when evaluating creatures");
    println!("            Options: [jump]");
    println!();
    println!("PLAYBACK OPTIONS:");
    println!("    -c, --creature <CREATURE_ID>");
    println!("            Playback a specific creature");
    println!();
    println!("    -g, --generation <GENERATION_ID>");
    println!("            Playback a specific generation");
    println!();
    println!("    -s, --session");
    println!("            Playback the entire session");
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
                } else if arg == "-t" || arg == "--test-time" {
                    train_config.test_time =
                        expect_res(expect(opts.next(), "Expected <TEST_TIME>")?.parse::<usize>(), "Invalid <TEST_TIME>")?;
                } else if arg == "-p" || arg == "--population" {
                    train_config.pop_size =
                        expect_res(expect(opts.next(), "Expected <POPULATION>")?.parse::<usize>(), "Invalid <POPULATION>")?;
                } else if arg == "-e" || arg == "--elitism" {
                    train_config.elitism = expect_res(expect(opts.next(), "Expected <ELITISM>")?.parse::<f32>(), "Invalid <ELITISM>")?;
                } else if arg == "-r" || arg == "--rand_percent" {
                    train_config.rand_percent =
                        expect_res(expect(opts.next(), "Expected <RAND_PERCENT>")?.parse::<f32>(), "Invalid <RAND_PERCENT>")?;
                } else if arg == "-f" || arg == "--fitness" {
                    let fun = expect(opts.next(), "Expected <FITNESS_FN>")?;
                    if fun == "jump" {
                        train_config.fitness_fn = fun.to_string();
                    } else {
                        return err("Invalid <FITNESS_FN>");
                    }
                }
            }
        }

        println!("Training with the following config: ");
        println!("    session = {}", train_config.session);
        println!("    visual = {}", train_config.visual);
        println!("    silent = {}", train_config.silent);
        println!("    test_time = {}", train_config.test_time);
        println!("    population = {}", train_config.pop_size);
        println!("    elitism = {}", train_config.elitism);
        println!("    rand_percent = {}", train_config.rand_percent);
        println!("    fitness = {}", train_config.fitness_fn);
        println!();

        train::train(train_config);
    } else if args[1] == "play" {
        let mut playback_config = PlaybackConfig { session: expect(args.get(2), "Expected [session]")?.clone(), ..Default::default() };
        let arg3 = expect(args.get(3), "Expected [-c|-g|-s]")?;

        if arg3 == "-c" || arg3 == "--creature" {
            playback_config.mode = PlaybackMode::Creature(expect_res(
                expect(args.get(4), "Expected <CREATURE_ID>")?.parse::<usize>(),
                "Invalid <CREATURE_ID>",
            )?);
        } else if arg3 == "-g" || arg3 == "--generation" {
            playback_config.mode = PlaybackMode::Generation(expect_res(
                expect(args.get(4), "Expected <GENERATION_ID>")?.parse::<usize>(),
                "Invalid <GENERATION_ID>",
            )?);
        } else if arg3 == "-s" || arg3 == "--session" {
            playback_config.mode = PlaybackMode::Session;
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
