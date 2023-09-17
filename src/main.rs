use std::{env, process};

mod rope_bridge;
mod rucksacks;
mod elf_crates;
mod callories;
mod rock_paper_scissors;
mod assignment;
mod marker;
mod directory;
mod tree_house;

fn main() {
    let exercise = parse_env_args().unwrap_or_else(|err| {
        println!("Problem parsing argumments {err}");
        process::exit(1);
    });

    if let Err(err) = exercise.run() {
        println!("Problem while running the exercise {err}");
        process::exit(1);
    }
}

enum Exercise {
    Callories(Part),
    RockPaperScissors(Part),
    Rucksack(Part),
    Assignement(Part),
    ElfCrates(Part),
    Marker(Part),
    Directory(Part),
    TreeHouse(Part),
    RopeBridge(Part)
}

impl Exercise {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Exercise::Callories(part) => {
                let result = match part {
                    Part::Part1 => callories::find_elf_with_most_callories_v2(),
                    Part::Part2 => callories::find_sum_of_three_most_callories_v2()
                };
                println!("Got {}", result);
            },
            Exercise::RockPaperScissors(part) => {
                let result = match part {
                    Part::Part1 => rock_paper_scissors::first_part::compute_score_with_initial_strategy(),
                    Part::Part2 => rock_paper_scissors::second_part::compute_score_with_second_strategy()
                };
                println!("Got {}", result);
            },
            Exercise::Rucksack(part) => {
                let result = match part {
                    Part::Part1 => rucksacks::first_part::compute_priorities_sum()?,
                    Part::Part2 => rucksacks::second_part::compute_priorities_sum()?
                };
                println!("Got {}", result);
            },
            Exercise::Assignement(part) => {
                let result = match part {
                    Part::Part1 => assignment::count_fully_contained_assignement_in_pair()?,
                    Part::Part2 => assignment::count_overlapping_assignement_in_pair()?
                };
                println!("Got {}", result);
            }
            Exercise::ElfCrates(part) => {
                let should_move_crate_one_at_the_time = match part {
                    Part::Part1 => true,
                    Part::Part2 => false
                };
                let result = elf_crates::move_crates(should_move_crate_one_at_the_time)?;
                println!("Got {}", result);
            },
            Exercise::Marker(part) => {
                let result = match part {
                    Part::Part1 => marker::find_start_of_packet_marker_index()?,
                    Part::Part2 => marker::find_start_of_message_marker_index()?
                };
                println!("Got {}", result);
            },
            Exercise::Directory(part) => {
                let result = match part {
                    Part::Part1 => directory::find_sum_of_small_diretories()?,
                    Part::Part2 => directory::find_smallest_dir_to_delete_for_update()?
                };
                println!("Got {}", result);
            },
            Exercise::TreeHouse(part) => {
                let result = match part {
                    Part::Part1 => tree_house::count_visible_trees()?,
                    Part::Part2 => tree_house::find_highest_scenic_score()?
                };
                println!("Got {}", result);
            },
            Exercise::RopeBridge(part) => {
                let knots_number = match part {
                    Part::Part1 => 2,
                    Part::Part2 => 10,
                };
                let result = rope_bridge::count_distinct_tail_positions(knots_number)?;
                println!("Got {}", result);
            }
        };
        Ok(())
    }
}

enum Part {
    Part1,
    Part2,
}

fn parse_env_args() -> Result<Exercise, Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err(format!("Invalid number of arguments, expected command as `cargo run <exercise name> <part (part_1 or part_2)>`, got arguments {:?}", args).into());
    }
    let exercise_name = args[1].as_str();
    let exercise_part = args[2].as_str();
    let part = match exercise_part {
        "part_1" => Part::Part1,
        "part_2" => Part::Part2,
        other => {
            return Err(format!("Unknown part chose, please choose either `part_1` or `part_2`, got {}", other).into());
        }
    };
    match exercise_name {
        "callories" => {
            Ok(Exercise::Callories(part))
        },
        "rock-paper-scissors" => {
            Ok(Exercise::RockPaperScissors(part))
        },
        "rucksack" => {
            Ok(Exercise::Rucksack(part))
        },
        "assignement" => {
            Ok(Exercise::Assignement(part))
        },
        "elf-crates" => {
            Ok(Exercise::ElfCrates(part))
        },
        "marker" => {
            Ok(Exercise::Marker(part))
        },
        "directory" => {
            Ok(Exercise::Directory(part))
        },
        "tree-house" => {
            Ok(Exercise::TreeHouse(part))
        },
        "rope-bridge" => {
            Ok(Exercise::RopeBridge(part))
        },
        other => {
            return Err(format!("Unknown exercise chosen, please choose one of the available exercise, got {}", other).into());
        }
    }
}