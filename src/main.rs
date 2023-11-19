use std::{env, process};

use advent_of_code::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = Command::try_from(&args).unwrap_or_else(|err| {
        println!("Problem parsing argumments {err}");
        process::exit(1);
    });

    if let Err(err) = command.run() {
        println!("Problem while running the exercise {err}");
        process::exit(1);
    }
}
