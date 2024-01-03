use std::{
    self,
    collections::{HashMap, HashSet},
};

use regex::Regex;

pub fn find_most_released_pressure(
    filename: &str,
    available_minutes: usize,
    number_of_actors: usize,
) -> Result<usize, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;
    let mut valves = HashMap::new();
    let mut worthy_valves_count = 0;
    let mut max_release_pressure_rate = 0;
    for line in content.lines() {
        println!("{line}");
        let valve = Valve::try_from(line)?;
        if valve.rate > 0 {
            worthy_valves_count += 1;
            max_release_pressure_rate += valve.rate;
        }
        valves.insert(valve.id.clone(), valve);
    }

    println!("Max release pressure rate: {max_release_pressure_rate}");

    let mut minutes = 1;

    let starting_valve_id = "AA";

    let mut path_index = 0;
    let mut paths = HashMap::new();
    paths.insert(
        path_index,
        VolcanoPath::new(starting_valve_id, number_of_actors),
    );

    // Opening path + positions at opening time
    let mut opening_paths: HashSet<String> = HashSet::new();
    // Ordered opening Path (sorted version of exact opening path + |<path addendum>) -> released_pressure until end given the opened valves
    let mut opening_records: HashMap<String, usize> = HashMap::new();

    let mut non_open_valves_rates_cache: HashMap<String, Vec<usize>> = HashMap::new();

    let mut maximum_released_pressure = 0;

    let mut iteration = 0;
    let mut iteration_per_minutes = 0;

    while minutes <= available_minutes {
        println!("#### MINUTE {minutes} ####");
        println!("Number of paths {}", paths.len());

        let mut paths_to_be_added = vec![];
        let mut paths_to_be_removed = vec![];

        let mut removed_paths = 0;

        for (i, path) in &mut paths {
            iteration_per_minutes += 1;
            path.accumulate_released_pressure();

            // Does take into account this turn
            // We start minutes at 1, so initially we are at remaining_minutes =  30 - 1 = 29;
            // At minute 15, we have remaining_minutes = 30 - 15 = 15;
            // At minute 29, we have remaining_minutes = 30 - 29 = 1;
            let remaining_minutes = available_minutes - minutes;

            let mut next_actor_paths = vec![path.clone()];

            // The existing path is removed in any case
            paths_to_be_removed.push(*i);

            // Choice of 10 is mostly based on experience here
            // It matches the time at which there are paths with enough open valves to overcome weak ones
            if maximum_released_pressure > 0 && minutes > 10 {
                let non_open_valves = non_open_valves_rates_cache.get(&path.opening_path);
                let mut ordered_non_open_valves_rates: Vec<usize> = vec![];
                match non_open_valves {
                    None => {
                        let mut non_open_valves_rates = vec![];
                        for id in valves.keys() {
                            if !path.opening_path.contains(id) {
                                non_open_valves_rates.push(valves.get(id).unwrap().rate);
                            }
                        }
                        non_open_valves_rates.sort_unstable();
                        for i in (0..non_open_valves_rates.len()).rev() {
                            ordered_non_open_valves_rates.push(non_open_valves_rates[i]);
                        }
                        non_open_valves_rates_cache.insert(
                            path.opening_path.clone(),
                            ordered_non_open_valves_rates.clone(),
                        );
                    }
                    Some(cached_value) => {
                        ordered_non_open_valves_rates = cached_value.clone();
                    }
                };
                let mut perfect_released_pressure =
                    path.released_pressure + path.released_pressure_rate * remaining_minutes;
                for i in 0..ordered_non_open_valves_rates.len() {
                    let minutes_to_open_valve = i / number_of_actors * 2;
                    let active_minutes = if remaining_minutes > minutes_to_open_valve {
                        remaining_minutes - minutes_to_open_valve
                    } else {
                        0
                    };
                    perfect_released_pressure += active_minutes * ordered_non_open_valves_rates[i];
                }
                if perfect_released_pressure < maximum_released_pressure {
                    // println!("Killing a not good enough path");
                    removed_paths += 1;
                    continue;
                } else {
                    // println!("Diff is {}, perfect released pressure {}", perfect_released_pressure - maximum_released_pressure, perfect_released_pressure);
                }
            }

            for actor_index in 0..number_of_actors {
                let mut actor_paths = next_actor_paths;
                next_actor_paths = vec![];

                // println!("[Path {i}] - [Actor {actor_index}] - Operate on #{} paths", actor_paths.len());

                for actor_path in &mut actor_paths {
                    let valve_can_be_opened = actor_path.can_open_valve(&valves, actor_index);

                    // If we can't open the valve, we create the new paths based on the moving possibilities
                    if !valve_can_be_opened {
                        // REMIND ME: Could filter out loops
                        actor_path.actors[actor_index]
                            .next_valves_possibilites(&valves)
                            .iter()
                            .map(|next_valve_id| {
                                let mut new_path = actor_path.clone();
                                new_path.move_to_new_valve(actor_index, &next_valve_id);
                                new_path
                            })
                            .for_each(|p| {
                                next_actor_paths.push(p);
                            });
                    } else {
                        // If we can open the valve,
                        // We first check that there are no already better path:
                        //  1. This exact opening path has already been made in a previous iteration, in this case, this path can not be better
                        //  2. If there is an existing opening record, we compare the released pressure until the end. If it is less than the existing record, this path can not be better
                        // We check if we are opening the last one, if yes, then we don't create new possibilities
                        // Else, we create one possibility for the path opening the current valve
                        // And we create the other possibilities by moving

                        let equivalent_addendumns = actor_path.actor_positions_addendums();
                        if equivalent_addendumns.iter().any(|a| {
                            let completed_opening_path =
                                actor_path.derive_next_exact_path(actor_index) + "|" + a;
                            opening_paths.contains(&completed_opening_path)
                        }) {
                            // println!("Already reached path {:?} in a previous iteration with another path. Closing this one.", completed_opening_path);
                            removed_paths += 1;
                            continue;
                        }

                        let incomplete_ordered_path =
                            actor_path.derive_next_incomplete_ordered_path(actor_index);
                        let mut should_be_removed = false;

                        for addendum in equivalent_addendumns {
                            let ordered_path =
                                incomplete_ordered_path.clone() + "|" + addendum.as_str();
                            if let Some(existing_record) = opening_records.get(&ordered_path) {
                                let released_pressure_until_end = actor_path.released_pressure
                                    + (actor_path.released_pressure_rate
                                        + valves
                                            .get(&actor_path.actors[actor_index].current_valve_id)
                                            .unwrap()
                                            .rate)
                                        * remaining_minutes;
                                if released_pressure_until_end <= *existing_record {
                                    should_be_removed = true;
                                    break;
                                } else {
                                    // println!("Path {ordered_path} is updated with stronger value. Previous record is removed");
                                    // Remind me: we should have a way to remove all the paths generated by the previous record
                                    opening_records
                                        .insert(ordered_path, released_pressure_until_end);
                                }
                            }
                        }
                        if should_be_removed {
                            // println!("Equal or better path already open in a previous iteration {i}. Released pressure: {} Path: {}", existing_record, ordered_path);
                            removed_paths += 1;
                            continue;
                        }

                        let is_opening_last_valve =
                            actor_path.open_valves_count() == worthy_valves_count - 1;
                        if is_opening_last_valve {
                            actor_path.open_valve(&valves, actor_index);
                            actor_path.stop(remaining_minutes);
                            opening_paths.insert(
                                actor_path.opening_path.clone()
                                    + "|"
                                    + actor_path.actor_positions_addendum().as_str(),
                            );
                            opening_records.insert(
                                actor_path.ordered_opening_path.clone()
                                    + "|"
                                    + actor_path.actor_positions_addendum().as_str(),
                                actor_path.released_pressure,
                            );
                            // println!("Path {i} - Path {} - All valves open with released pressure: {}", actor_path.opening_path, actor_path.released_pressure);
                            if actor_path.released_pressure > maximum_released_pressure {
                                println!(
                                    "Path {i} - Path {} - Found new maximum at: {}. Rate {}",
                                    actor_path.opening_path,
                                    actor_path.released_pressure,
                                    actor_path.released_pressure_rate
                                );
                                maximum_released_pressure = actor_path.released_pressure;
                            }
                            removed_paths += 1;
                            continue;
                        }

                        actor_path.actors[actor_index]
                            .next_valves_possibilites(&valves)
                            .iter()
                            .map(|next_valve_id| {
                                let mut new_path = actor_path.clone();
                                new_path.move_to_new_valve(actor_index, &next_valve_id);
                                new_path
                            })
                            .for_each(|p| {
                                next_actor_paths.push(p);
                            });

                        let mut opening_valve_possibility = actor_path.clone();
                        opening_valve_possibility.open_valve(&valves, actor_index);
                        opening_paths.insert(
                            opening_valve_possibility.opening_path.clone()
                                + "|"
                                + opening_valve_possibility
                                    .actor_positions_addendum()
                                    .as_str(),
                        );
                        let released_pressure_until_end = opening_valve_possibility
                            .released_pressure
                            + opening_valve_possibility.released_pressure_rate * remaining_minutes;
                        opening_records.insert(
                            opening_valve_possibility.ordered_opening_path.clone(),
                            released_pressure_until_end,
                        );
                        if released_pressure_until_end > maximum_released_pressure {
                            maximum_released_pressure = released_pressure_until_end;
                        }
                        next_actor_paths.push(opening_valve_possibility);
                    }
                }
            }

            // The derived are added
            for p in next_actor_paths {
                paths_to_be_added.push(p);
            }
        }

        for i in paths_to_be_removed {
            paths.remove(&i);
        }

        // println!("Adding {} paths", paths_to_be_added.len());
        for path_to_be_added in paths_to_be_added {
            // println!("Has already {} valves open: ", {path_to_be_added.open_valves.len()});
            path_index += 1;
            paths.insert(path_index, path_to_be_added);
        }

        println!("Iteration in the minute {iteration_per_minutes}");
        println!("Removed paths in the minute {}", removed_paths);
        println!("\n");
        iteration += iteration_per_minutes;
        iteration_per_minutes = 0;
        minutes += 1;
    }

    for p in paths.values() {
        if p.released_pressure > maximum_released_pressure {
            println!(
                "Found one at the end! {}, #{}",
                p.released_pressure,
                p.open_valves_count()
            );
            maximum_released_pressure = p.released_pressure;
        }
    }

    println!("Iteration: {iteration}");

    Ok(maximum_released_pressure)
}

#[derive(Clone)]
struct VolcanoPath {
    released_pressure_rate: usize,
    released_pressure: usize,
    stopped: bool,
    opening_path: String,
    ordered_opening_path: String,
    actors: Vec<ActorPath>,
}

#[derive(Clone)]
struct ActorPath {
    visited_valves_since_last_open: String,
    current_valve_id: String,
}

impl ActorPath {
    fn next_valves_possibilites(&self, valves: &HashMap<String, Valve>) -> Vec<String> {
        let valve = valves.get(&self.current_valve_id).unwrap();
        valve
            .connected_valves
            .iter()
            .filter_map(|id| {
                if self.visited_valves_since_last_open.contains(id) {
                    None
                } else {
                    Some(id.clone())
                }
            })
            .collect()
    }
}

impl VolcanoPath {
    fn new(starting_id: &str, number_of_actors: usize) -> Self {
        VolcanoPath {
            released_pressure_rate: 0,
            released_pressure: 0,
            stopped: false,
            opening_path: "".to_owned(),
            ordered_opening_path: "".to_owned(),
            actors: vec![
                ActorPath {
                    visited_valves_since_last_open: starting_id.to_owned(),
                    current_valve_id: starting_id.to_owned(),
                };
                number_of_actors
            ],
        }
    }
    fn derive_next_exact_path(&self, actor_index: usize) -> String {
        if self.opening_path.len() == 0 {
            return self.actors[actor_index].current_valve_id.clone();
        }
        self.opening_path.clone() + "-" + self.actors[actor_index].current_valve_id.as_str()
    }

    fn derive_next_incomplete_ordered_path(&self, actor_index: usize) -> String {
        if self.opening_path.len() == 0 {
            return self.actors[actor_index].current_valve_id.clone();
        }
        let a =
            self.opening_path.clone() + "-" + self.actors[actor_index].current_valve_id.as_str();
        let mut b = a.split("-").collect::<Vec<&str>>();
        b.sort_unstable();
        b.join("-")
    }

    fn derive_ordered_path(&self) -> String {
        if self.opening_path.len() == 0 {
            return self.opening_path.clone();
        }
        let mut b = self.opening_path.split("-").collect::<Vec<&str>>();
        b.sort_unstable();
        b.join("-") + "|" + self.actor_positions_addendum().as_str()
    }

    fn actor_positions_addendum(&self) -> String {
        match self.actors.len() {
            0 => "".to_owned(),
            1 => self.actors[0].current_valve_id.clone(),
            _ => {
                let a = self
                    .actors
                    .iter()
                    .map(|a| a.current_valve_id.clone())
                    .collect::<Vec<String>>();
                a.join("_")
            }
        }
    }

    fn actor_positions_addendums(&self) -> Vec<String> {
        match self.actors.len() {
            0 => vec!["".to_owned()],
            1 => vec![self.actors[0].current_valve_id.clone()],
            2 => {
                let mut a = self
                    .actors
                    .iter()
                    .map(|a| a.current_valve_id.clone())
                    .collect::<Vec<String>>();
                let mut possibilities = vec![a.join("_")];
                a.swap(0, 1);
                possibilities.push(a.join("_"));
                possibilities
            }
            _ => {
                let a = self
                    .actors
                    .iter()
                    .map(|a| a.current_valve_id.clone())
                    .collect::<Vec<String>>();
                // @dev permutations not implemented here
                vec![a.join("_")]
            }
        }
    }

    fn accumulate_released_pressure(&mut self) {
        self.released_pressure += self.released_pressure_rate;
    }

    fn can_open_valve(&self, valves: &HashMap<String, Valve>, actor_index: usize) -> bool {
        let valve = valves
            .get(&self.actors[actor_index].current_valve_id)
            .unwrap();
        valve.rate > 0
            && !self
                .opening_path
                .contains(&self.actors[actor_index].current_valve_id)
    }

    fn open_valves_count(&self) -> usize {
        if self.opening_path.len() == 0 {
            return 0;
        }
        self.opening_path.split("-").count()
    }

    fn open_valve(&mut self, valves: &HashMap<String, Valve>, actor_index: usize) {
        let rate = valves
            .get(&self.actors[actor_index].current_valve_id)
            .unwrap()
            .rate;
        self.released_pressure_rate += rate;
        self.opening_path = self.derive_next_exact_path(actor_index);
        self.ordered_opening_path = self.derive_ordered_path();
        // Actor specific
        self.actors[actor_index].visited_valves_since_last_open = self.actors[actor_index].current_valve_id.clone();
    }

    fn move_to_new_valve(&mut self, actor_index: usize, new_valve_id: &str) {
        self.actors[actor_index]
            .visited_valves_since_last_open += "-";
        self.actors[actor_index]
            .visited_valves_since_last_open += new_valve_id;
        self.actors[actor_index].current_valve_id = new_valve_id.to_owned();
    }

    fn stop(&mut self, remaining_minutes: usize) {
        self.released_pressure += remaining_minutes * self.released_pressure_rate;
        self.stopped = true;
    }
}

impl From<&VolcanoPath> for VolcanoPath {
    fn from(value: &VolcanoPath) -> Self {
        value.clone()
    }
}

struct Valve {
    id: String,
    rate: usize,
    connected_valves: Vec<String>,
}

impl TryFrom<&str> for Valve {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let pattern = Regex::new(
            r"Valve ([A-Z]{2}) has flow rate=(\d+);(?: tunnels lead to valves ((?:[A-Z]{2}, )*[A-Z]{2}))?(?: tunnel leads to valve ([A-Z]{2}))?",
        )?;
        let captures = pattern.captures(value)
            .ok_or(format!("Unable to parse line into sensor, expected line of format `Valve <double capital letter value> has flow rate=<usize value>; tunnel(s) lead to valve(s) <comma separated list of double capital letter values>`. Got `{}`", value))?;
        if captures.len() < 3 {
            return Err(format!("Unable to parse line into sensor, expected line of format `Valve <double capital letter value> has flow rate=<usize value>; tunnel(s) lead to valve(s) <comma separated list of double capital letter values>`. Got `{}`", value).into());
        }
        let valve_id = captures[1].to_owned();
        let rate = captures[2].parse::<usize>().map_err(|e| {
            format!(
                "Invalid numerical value for the `rate` of the valve. Got err {}",
                e
            )
        })?;

        let connected_valves = if let Some(valves_list) = captures.get(3) {
            valves_list
                .as_str()
                .split(", ")
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
        } else if let Some(single_valve) = captures.get(4) {
            vec![single_valve.as_str().to_owned()]
        } else {
            return Err(format!("Need some tunnels for valve {valve_id}").into());
        };

        Ok(Valve {
            id: valve_id,
            rate,
            connected_valves,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1_should_give_expected_result() {
        assert_eq!(
            find_most_released_pressure("inputs/input-16-example.txt", 30, 1).unwrap(),
            1651
        );
    }

    #[test]
    fn part_1_should_give_expected_result() {
        assert_eq!(
            find_most_released_pressure("inputs/input-16.txt", 30, 1).unwrap(),
            2181
        );
    }

    #[test]
    fn example_part_2_should_give_expected_result() {
        assert_eq!(
            find_most_released_pressure("inputs/input-16-example.txt", 26, 2).unwrap(),
            1707
        );
    }

    #[test]
    fn part_2_should_give_expected_result() {
        assert_eq!(
            find_most_released_pressure("inputs/input-16.txt", 26, 2).unwrap(),
            2824
        );
    }
}
