use std::{
    self,
    collections::{HashMap, HashSet},
};

use regex::Regex;

pub fn find_most_released_pressure(filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;
    let mut valves = HashMap::new();
    let mut worthy_valves_count = 0;
    for line in content.lines() {
        println!("{line}");
        let valve = Valve::try_from(line)?;
        if valve.rate > 0 {
            worthy_valves_count += 1;
        }
        valves.insert(valve.id.clone(), valve);
    }

    let mut minutes = 1;

    let starting_valve_id = "AA";

    let mut path_index = 0;
    let mut paths = HashMap::new();
    paths.insert(path_index, VolcanoPath::new(starting_valve_id));

    // Exact opening path
    let mut opening_paths: HashSet<String> = HashSet::new();
    // Ordered opening Path (sorted version of exact opening path + _<current valve id>) -> (released_pressure until end given the opened valves, path index)
    let mut opening_records: HashMap<String, (usize, usize)> = HashMap::new();

    let mut maximum_released_pressure = 0;

    let mut iteration = 0;
    let mut iteration_per_minutes = 0;

    while minutes <= 30 {
        println!("#### MINUTE {minutes} ####");

        let mut paths_to_be_added = vec![];
        let mut paths_to_be_removed = vec![];

        for (i, path) in &mut paths {
            iteration_per_minutes += 1;
            path.accumulate_released_pressure();

            // Does take into account this turn
            // We start minutes at 1, so initially we are at remaining_minutes =  30 - 1 = 29;
            // At minute 15, we have remaining_minutes = 30 - 15 = 15;
            // At minute 29, we have remaining_minutes = 30 - 29 = 1;
            let remaining_minutes = 30 - minutes;

            let valve_can_be_opened = path.can_open_valve(&valves);

            if valve_can_be_opened {
                if opening_paths.contains(&path.derive_next_exact_path()) {
                    // println!("Already reached path {:?}. Closing this one.", path.open_valves);
                    paths_to_be_removed.push(*i);
                    continue;
                }
                let ordered_path = path.derive_next_ordered_path();
                let record = opening_records.get(&ordered_path);
                if let Some(existing_record) = record {
                    let released_pressure_until_end = path.released_pressure
                        + (path.released_pressure_rate
                            + valves.get(&path.current_valve_id).unwrap().rate)
                            * remaining_minutes;
                    if released_pressure_until_end <= existing_record.0 {
                        println!("Equal or better path already open in a previous iteration {i}. Released pressure: {} Path: {}", existing_record.0, ordered_path);
                        paths_to_be_removed.push(*i);
                        continue;
                    } else {
                        // println!("Path {ordered_path} is updated with stronger value. Previous record is removed");
                        paths_to_be_removed.push(existing_record.1);
                        opening_records.insert(ordered_path, (released_pressure_until_end, *i));
                    }
                }
            }

            let is_opening_last_valve =
                valve_can_be_opened && path.open_valves.len() == worthy_valves_count - 1;
            let has_open_all_valves = path.open_valves.len() == worthy_valves_count;
            if !valve_can_be_opened || !is_opening_last_valve || !has_open_all_valves {
                let other_possibilites = path.next_valves_possibilites(&valves);
                for next_valve_id in other_possibilites {
                    let mut new_path = path.clone();
                    new_path.move_to_new_valve(&next_valve_id);
                    paths_to_be_added.push(new_path);
                }
            }

            if valve_can_be_opened {
                // println!("[Path {i}] - Open valve {}
                // Exact path {}
                // Ordered path {}", path.current_valve_id, path.opening_path, path.ordered_opening_path);
                path.open_valve(&valves);
                opening_paths.insert(path.opening_path.clone());
                let released_pressure_until_end =
                    path.released_pressure + path.released_pressure_rate * remaining_minutes;
                opening_records.insert(
                    path.ordered_opening_path.clone(),
                    (released_pressure_until_end, *i),
                );
            } else {
                // println!("No choice for path: {i}");
                path.stop(remaining_minutes);
                if path.released_pressure > maximum_released_pressure {
                    // println!("Path {i} - Found new maximum at: {}", path.released_pressure);
                    maximum_released_pressure = path.released_pressure;
                }
                paths_to_be_removed.push(*i);
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
        println!("\n");
        iteration += iteration_per_minutes;
        iteration_per_minutes = 0;
        minutes += 1;
    }

    for p in paths.values() {
        if p.released_pressure > maximum_released_pressure {
            println!("Found one at the end! {}", p.released_pressure);
            maximum_released_pressure = p.released_pressure;
        }
    }

    println!("Iteration: {iteration}");

    Ok(maximum_released_pressure)
}

#[derive(Clone)]
struct VolcanoPath {
    visited_valves_since_last_open: HashSet<String>,
    open_valves: HashSet<String>,
    current_valve_id: String,
    released_pressure_rate: usize,
    released_pressure: usize,
    stopped: bool,
    opening_path: String,
    ordered_opening_path: String,
}

impl VolcanoPath {
    fn new(starting_id: &str) -> Self {
        VolcanoPath {
            visited_valves_since_last_open: HashSet::new(),
            open_valves: HashSet::new(),
            current_valve_id: starting_id.to_owned(),
            released_pressure_rate: 0,
            released_pressure: 0,
            stopped: false,
            opening_path: "".to_owned(),
            ordered_opening_path: "".to_owned(),
        }
    }
    fn derive_next_exact_path(&self) -> String {
        if self.opening_path.len() == 0 {
            return self.current_valve_id.clone();
        }
        self.opening_path.clone() + "-" + self.current_valve_id.as_str()
    }

    fn derive_next_ordered_path(&self) -> String {
        if self.opening_path.len() == 0 {
            return self.current_valve_id.clone();
        }
        let a = self.opening_path.clone() + "-" + self.current_valve_id.as_str();
        let mut b = a.split("-").collect::<Vec<&str>>();
        b.sort_unstable();
        b.join("-") + "_" + self.current_valve_id.as_str()
    }

    fn derive_ordered_path(&self) -> String {
        if self.opening_path.len() == 0 {
            return self.opening_path.clone();
        }
        let mut b = self.opening_path.split("-").collect::<Vec<&str>>();
        b.sort_unstable();
        b.join("-") + "_" + self.current_valve_id.as_str()
    }

    fn accumulate_released_pressure(&mut self) {
        self.released_pressure += self.released_pressure_rate;
    }

    fn can_open_valve(&self, valves: &HashMap<String, Valve>) -> bool {
        let valve = valves.get(&self.current_valve_id).unwrap();
        valve.rate > 0 && !self.open_valves.contains(&self.current_valve_id)
    }

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

    fn move_to_new_valve(&mut self, new_valve_id: &str) {
        self.visited_valves_since_last_open
            .insert(new_valve_id.to_owned());
        self.current_valve_id = new_valve_id.to_owned();
    }

    fn open_valve(&mut self, valves: &HashMap<String, Valve>) {
        let rate = valves.get(&self.current_valve_id).unwrap().rate;
        self.open_valves.insert(self.current_valve_id.clone());
        self.opening_path = self.derive_next_exact_path();
        self.ordered_opening_path = self.derive_ordered_path();
        self.released_pressure_rate += rate;
        let mut visited_valves_since_last_open = HashSet::new();
        visited_valves_since_last_open.insert(self.current_valve_id.clone());
        self.visited_valves_since_last_open = visited_valves_since_last_open;

        let non_open_valves = valves
            .iter()
            .filter(|(v_id, v)| !self.open_valves.contains(*v_id) && v.rate > 0)
            .count();
        if non_open_valves == 0 {
            println!("OPEN ALL THE VALVES!");
        }
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
            find_most_released_pressure("inputs/input-16-example.txt").unwrap(),
            1651
        );
    }

    #[test]
    fn part_1_should_give_expected_result() {
        assert_eq!(
            find_most_released_pressure("inputs/input-16.txt").unwrap(),
            2181
        );
    }
}
