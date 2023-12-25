use std::{
    self,
    collections::{HashMap, HashSet},
};

use regex::Regex;

pub fn find_most_released_pressure(filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;
    let mut valves = HashMap::new();
    for line in content.lines() {
        println!("{line}");
        let valve = Valve::try_from(line)?;
        valves.insert(valve.id.clone(), valve);
    }

    let mut minutes = 1;

    let starting_valve_id = "AA";

    let mut path_index = 0;
    let mut paths = HashMap::new();
    paths.insert(path_index, VolcanoPath::new(starting_valve_id));

    let mut max = 0;

    while minutes <= 30 {
        println!("#### MINUTE {minutes} ####");

        let mut paths_to_be_added = vec![];
        let mut paths_to_be_removed = vec![];

        for (i, path) in &mut paths {
            let other_possibilites = path.next_valves_possibilites(&valves);

            path.accumulate_released_pressure();

            for next_valve_id in other_possibilites {
                let mut new_path = path.clone();
                new_path.move_to_new_valve(&next_valve_id);
                paths_to_be_added.push(new_path);
            }

            if path.can_open_valve(&valves) {
                println!("[Path {i}] - Open valve {}", path.current_valve_id);
                path.open_valve(&valves);
            } else {
                println!("No choice for path: {i}");
                let remaining_minutes = if minutes + 1 > 30 {
                    0
                } else {
                    30 - minutes - 1
                };
                path.stop(remaining_minutes);
                if path.released_pressure > max {
                    max = path.released_pressure;
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

        println!("\n");
        minutes += 1;
    }

    for p in paths.values() {
        if p.released_pressure > max {
            max = p.released_pressure;
        }
    }

    Ok(max)
}

#[derive(Clone)]
struct VolcanoPath {
    visited_valves_since_last_open: HashSet<String>,
    open_valves: HashSet<String>,
    current_valve_id: String,
    released_pressure_rate: usize,
    released_pressure: usize,
    stopped: bool,
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
        }
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
        self.released_pressure_rate += rate;
        let mut visited_valves_since_last_open = HashSet::new();
        visited_valves_since_last_open.insert(self.current_valve_id.clone());
        self.visited_valves_since_last_open = visited_valves_since_last_open;
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
}
