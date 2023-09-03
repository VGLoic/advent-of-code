use std::{env, process};

mod rope_bridge;

fn main() {
    let exercise = parse_env_args().unwrap_or_else(|err| {
        println!("Problem parsing argumments {err}");
        process::exit(1);
    });

    let _ = exercise.run().unwrap_or_else(|err| {
        println!("Problem while running the exercise {err}");
        process::exit(1);
    });
}

enum Exercise {
    RopeBridge(Part),
}

impl Exercise {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Exercise::RopeBridge(_) => {
                let result = rope_bridge::count_distinct_tail_positions()?;
                println!("Got {}", result);
                Ok(())
            }
        }
    }
}

enum Part {
    Part1,
}

fn parse_env_args() -> Result<Exercise, Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 || args.len() > 2 {
        return Err(format!("Invalid number of arguments, expected at least one and at most two, got {}", args.len()).into());
    }
    let exercise_name = args[1].as_str();
    match exercise_name {
        "rope-bridge" => {
            Ok(Exercise::RopeBridge(Part::Part1))
        },
        other => {
            return Err(format!("Unknown exercise chosen, please choose one of the available exercise, got {}", other).into());
        }
    }
}