use std::{env, process};

use advent_of_code::Exercise;


fn main() {
    let args: Vec<String> = env::args().collect();
    let exercise = Exercise::try_from(&args).unwrap_or_else(|err| {
        println!("Problem parsing argumments {err}");
        process::exit(1);
    });

    if let Err(err) = exercise.run() {
        println!("Problem while running the exercise {err}");
        process::exit(1);
    }
}