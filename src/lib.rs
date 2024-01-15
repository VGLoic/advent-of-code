mod assignment;
mod beacon_exclusion_zone;
mod boiling_boulders;
mod callories;
mod cathod_ray_tube;
mod directory;
mod distress_signal;
mod elf_crates;
mod hill_climbing;
mod marker;
mod monkey_in_the_middle;
mod proboscidea_volcanium;
mod pyroclastic_flow;
mod regolith_reservoir;
mod rock_paper_scissors;
mod rope_bridge;
mod rucksacks;
mod tree_house;

pub enum Command {
    Help,
    Exercise(Exercise),
}

impl Command {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Command::Help => {
                println!(
                    "Advent of code, edition 2022

Website: https://adventofcode.com/2022

List exercises and help: cargo run help

Usage: cargo run [exercise] [part] [ARGS]...

Exercise list (in the ascending order):
    - callories,
    - rock-paper-scissors,
    - rucksack,
    - assignement,
    - elf-creates,
    - marker,
    - directory,
    - tree-house,
    - rope-bridge,
    - cathod-ray-tube,
    - monkey-in-the-middle,
    - hill-climbing,
    - distress-signals,
    - regolith-reservoir,
    - beacon-exclusion-zone,
    - proboscidea-volcanium,
    - pyroclastic-flow,
    - boiling-boulders.

Part:
    - part_1,
        Run part 1 of the exercise
    - part_2
        Run part 2 of the exercise

Args:
    -ex, --example
        Run the exercise using exercise input instead of official input
    -h, --help
        List exercises and help
                "
                );
                Ok(())
            }
            Command::Exercise(exercise) => exercise.run(),
        }
    }
}

impl TryFrom<&Vec<String>> for Command {
    type Error = Box<dyn std::error::Error>;

    fn try_from(args: &Vec<String>) -> Result<Self, Self::Error> {
        if args.len() == 0 {
            return Err(
                format!(
                    "Invalid number of arguments, expected command as `cargo run <exercise name> <part (part_1 or part_2)>`, got no arguments",
                ).into()
            );
        }

        let help_cmd = "help".to_string();
        let short_help = "-h".to_string();
        let long_help = "--help".to_string();
        if args.contains(&long_help) | args.contains(&short_help) {
            return Ok(Command::Help);
        }

        if args[1] == help_cmd {
            return Ok(Command::Help);
        }

        return Exercise::try_from(args).and_then(|ex| Ok(Command::Exercise(ex)));
    }
}

pub enum Exercise {
    Callories(Part, bool),
    RockPaperScissors(Part, bool),
    Rucksack(Part, bool),
    Assignement(Part, bool),
    ElfCrates(Part, bool),
    Marker(Part, bool),
    Directory(Part, bool),
    TreeHouse(Part, bool),
    RopeBridge(Part, bool),
    CathodRayTube(Part, bool),
    MonkeyInTheMiddle(Part, bool),
    HillClimbing(Part, bool),
    DistressSignals(Part, bool),
    RegolithReservoir(Part, bool),
    BeaconExclusionZone(Part, bool),
    ProboscideaVolcanium(Part, bool),
    PyroclasticFlow(Part, bool),
    BoilingBoulders(Part, bool),
}

pub enum Part {
    Part1,
    Part2,
}

impl TryFrom<&Vec<String>> for Exercise {
    type Error = Box<dyn std::error::Error>;

    fn try_from(args: &Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
        if args.len() < 3 {
            return Err(
                format!(
                    "Invalid number of arguments, expected command as `cargo run <exercise name> <part>`, got arguments {:?}",
                    args
                ).into()
            );
        }
        let exercise_name = args[1].as_str();
        let exercise_part = args[2].as_str();
        let part = match exercise_part {
            "part_1" => Part::Part1,
            "part_2" => Part::Part2,
            other => {
                return Err(format!(
                    "Unknown part chose, please choose either `part_1` or `part_2`, got {}",
                    other
                )
                .into());
            }
        };

        let example_short = "-ex".to_string();
        let example_long = "--example".to_string();
        let use_example = args[3..].contains(&example_short) || args[3..].contains(&example_long);

        match exercise_name {
            "callories" => Ok(Exercise::Callories(part, use_example)),
            "rock-paper-scissors" => Ok(Exercise::RockPaperScissors(part, use_example)),
            "rucksack" => Ok(Exercise::Rucksack(part, use_example)),
            "assignement" => Ok(Exercise::Assignement(part, use_example)),
            "elf-crates" => Ok(Exercise::ElfCrates(part, use_example)),
            "marker" => Ok(Exercise::Marker(part, use_example)),
            "directory" => Ok(Exercise::Directory(part, use_example)),
            "tree-house" => Ok(Exercise::TreeHouse(part, use_example)),
            "rope-bridge" => Ok(Exercise::RopeBridge(part, use_example)),
            "cathod-ray-tube" => Ok(Exercise::CathodRayTube(part, use_example)),
            "monkey-in-the-middle" => Ok(Exercise::MonkeyInTheMiddle(part, use_example)),
            "hill-climbing" => Ok(Exercise::HillClimbing(part, use_example)),
            "distress-signals" => Ok(Exercise::DistressSignals(part, use_example)),
            "regolith-reservoir" => Ok(Exercise::RegolithReservoir(part, use_example)),
            "beacon-exclusion-zone" => Ok(Exercise::BeaconExclusionZone(part, use_example)),
            "proboscidea-volcanium" => Ok(Exercise::ProboscideaVolcanium(part, use_example)),
            "pyroclastic-flow" => Ok(Exercise::PyroclasticFlow(part, use_example)),
            "boiling-boulders" => Ok(Exercise::BoilingBoulders(part, use_example)),
            other => {
                return Err(format!(
                    "Unknown exercise chosen, please choose one of the available exercise, got {}",
                    other
                )
                .into());
            }
        }
    }
}

impl Exercise {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Exercise::Callories(part, use_example) => {
                let filename = if *use_example {
                    "inputs/input-01-example.txt"
                } else {
                    "inputs/input-01.txt"
                };
                let result = match part {
                    Part::Part1 => callories::find_max_callories_on_single_elf(filename)?,
                    Part::Part2 => callories::find_sum_of_maximums_callories(filename, 3)?,
                };
                println!("Got {}", result);
            }
            Exercise::RockPaperScissors(part, use_example) => {
                let filename = if *use_example {
                    "inputs/input-02-example.txt"
                } else {
                    "inputs/input-02.txt"
                };
                let result = match part {
                    Part::Part1 => {
                        rock_paper_scissors::compute_score_with_initial_strategy(filename)?
                    }
                    Part::Part2 => {
                        rock_paper_scissors::compute_score_with_second_strategy(filename)?
                    }
                };
                println!("Got {}", result);
            }
            Exercise::Rucksack(part, use_example) => {
                let filename = if *use_example {
                    "inputs/input-03-example.txt"
                } else {
                    "inputs/input-03.txt"
                };
                let result = match part {
                    Part::Part1 => rucksacks::first_part::compute_priorities_sum(filename)?,
                    Part::Part2 => rucksacks::second_part::compute_priorities_sum(filename)?,
                };
                println!("Got {}", result);
            }
            Exercise::Assignement(part, use_example) => {
                let filename = if *use_example {
                    "inputs/input-04-example.txt"
                } else {
                    "inputs/input-04.txt"
                };
                let result = match part {
                    Part::Part1 => assignment::count_fully_contained_assignement_in_pair(filename)?,
                    Part::Part2 => assignment::count_overlapping_assignement_in_pair(filename)?,
                };
                println!("Got {}", result);
            }
            Exercise::ElfCrates(part, use_example) => {
                let filename = if *use_example {
                    "inputs/input-05-example.txt"
                } else {
                    "inputs/input-05.txt"
                };
                let should_move_crate_one_at_the_time = match part {
                    Part::Part1 => true,
                    Part::Part2 => false,
                };
                let result = elf_crates::move_crates(filename, should_move_crate_one_at_the_time)?;
                println!("Got {}", result);
            }
            Exercise::Marker(part, use_example) => {
                let filename = if *use_example {
                    "inputs/input-06-example.txt"
                } else {
                    "inputs/input-06.txt"
                };
                let target_length = match part {
                    Part::Part1 => 4,
                    Part::Part2 => 14,
                };
                let result = marker::find_start_of_packet_marker_index(filename, target_length)?;
                println!("Got {}", result);
            }
            Exercise::Directory(part, use_example) => {
                let filename = if *use_example {
                    "inputs/input-07-example.txt"
                } else {
                    "inputs/input-07.txt"
                };
                let result = match part {
                    Part::Part1 => directory::find_sum_of_small_diretories(filename)?,
                    Part::Part2 => directory::find_smallest_dir_to_delete_for_update(filename)?,
                };
                println!("Got {}", result);
            }
            Exercise::TreeHouse(part, use_example) => {
                let filename = if *use_example {
                    "inputs/input-08-example.txt"
                } else {
                    "inputs/input-08.txt"
                };
                let result = match part {
                    Part::Part1 => tree_house::count_visible_trees(filename)?,
                    Part::Part2 => tree_house::find_highest_scenic_score(filename)?,
                };
                println!("Got {}", result);
            }
            Exercise::RopeBridge(part, use_example) => {
                let filename = match part {
                    Part::Part1 => {
                        if *use_example {
                            "inputs/input-09-example.txt"
                        } else {
                            "inputs/input-09.txt"
                        }
                    }
                    Part::Part2 => {
                        if *use_example {
                            "inputs/input-09-example-part-2.txt"
                        } else {
                            "inputs/input-09.txt"
                        }
                    }
                };
                let knots_number = match part {
                    Part::Part1 => 2,
                    Part::Part2 => 10,
                };
                let result = rope_bridge::count_distinct_tail_positions(filename, knots_number)?;
                println!("Got {}", result);
            }
            Exercise::CathodRayTube(part, use_example) => {
                let filename = if *use_example {
                    "inputs/input-10-example.txt"
                } else {
                    "inputs/input-10.txt"
                };
                let result = match part {
                    Part::Part1 => cathod_ray_tube::sum_signal_strengths(filename)?.to_string(),
                    Part::Part2 => cathod_ray_tube::display_signal(filename)?,
                };

                println!("Got \n{}", result)
            }
            Exercise::MonkeyInTheMiddle(part, use_example) => {
                let filename = if *use_example {
                    "inputs/input-11-example.txt"
                } else {
                    "inputs/input-11.txt"
                };
                let result = match part {
                    Part::Part1 => monkey_in_the_middle::compute_monkey_business(filename)?,
                    Part::Part2 => monkey_in_the_middle::compute_big_monkey_business(filename)?,
                };

                println!("Got {}", result)
            }
            Exercise::HillClimbing(part, use_example) => {
                let filename = if *use_example {
                    "inputs/input-12-example.txt"
                } else {
                    "inputs/input-12.txt"
                };
                let result = match part {
                    Part::Part1 => hill_climbing::find_shortest_path(filename)?,
                    Part::Part2 => {
                        hill_climbing::find_shortest_path_from_any_lowest_point(filename)?
                    }
                };

                println!("Got {}", result)
            }
            Exercise::DistressSignals(part, use_example) => {
                let filename = if *use_example {
                    "inputs/input-13-example.txt"
                } else {
                    "inputs/input-13.txt"
                };
                let result = match part {
                    Part::Part1 => distress_signal::sum_over_right_pair_indices(filename)?,
                    Part::Part2 => distress_signal::find_decoder_key(filename)?,
                };

                println!("Got {}", result)
            }
            Exercise::RegolithReservoir(part, use_example) => {
                let filename = if *use_example {
                    "inputs/input-14-example.txt"
                } else {
                    "inputs/input-14.txt"
                };
                let result = match part {
                    Part::Part1 => regolith_reservoir::find_number_of_resting_units_of_sand_before_falling_in_void(filename)?,
                    Part::Part2 => regolith_reservoir::find_number_of_resting_units_of_sand_before_blocked(filename)?,
                };

                println!("Got {}", result)
            }
            Exercise::BeaconExclusionZone(part, use_example) => {
                let filename = if *use_example {
                    "inputs/input-15-example.txt"
                } else {
                    "inputs/input-15.txt"
                };
                let row = if *use_example { 10 } else { 2_000_000 };
                let result = match part {
                    Part::Part1 => beacon_exclusion_zone::find_number_of_covered_positions_in_row(
                        filename, row,
                    )?,
                    Part::Part2 => {
                        beacon_exclusion_zone::find_distress_beacon_tuning_frequency(filename)?
                    }
                };
                println!("Got {}", result)
            }
            Exercise::ProboscideaVolcanium(part, use_example) => {
                let filename = if *use_example {
                    "inputs/input-16-example.txt"
                } else {
                    "inputs/input-16.txt"
                };
                let result = match part {
                    Part::Part1 => {
                        proboscidea_volcanium::find_most_released_pressure(filename, 30, 1)?
                    }
                    Part::Part2 => {
                        proboscidea_volcanium::find_most_released_pressure(filename, 26, 2)?
                    }
                };
                println!("Got {}", result)
            }
            Exercise::PyroclasticFlow(part, use_example) => {
                let filename = if *use_example {
                    "inputs/input-17-example.txt"
                } else {
                    "inputs/input-17.txt"
                };
                let result = match part {
                    Part::Part1 => pyroclastic_flow::find_tower_height(filename, 2_022)?,
                    Part::Part2 => {
                        pyroclastic_flow::find_tower_height(filename, 1_000_000_000_000)?
                    }
                };
                println!("Got {}", result)
            }
            Exercise::BoilingBoulders(part, use_example) => {
                let filename = if *use_example {
                    "inputs/input-18-example.txt"
                } else {
                    "inputs/input-18.txt"
                };
                let result = match part {
                    Part::Part1 => boiling_boulders::derive_surface_area(filename, false)?,
                    Part::Part2 => boiling_boulders::derive_surface_area(filename, true)?,
                };
                println!("Got {}", result)
            }
        };
        Ok(())
    }
}
