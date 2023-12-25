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

    // Opening Path -> released_pressure
    let mut opening_records: HashMap<String, usize> = HashMap::new();

    let mut maximum_released_pressure = 0;
    // Does not work
    // let mut maximum_potential_released_pressure = 0;

    while minutes <= 30 {
        println!("#### MINUTE {minutes} ####");
        // println!("Maximum released pressure {maximum_released_pressure}");

        let mut paths_to_be_added = vec![];
        let mut paths_to_be_removed = vec![];

        for (i, path) in &mut paths {
            path.accumulate_released_pressure();

            let remaining_minutes = if minutes + 1 > 30 {
                0
            } else {
                30 - minutes - 1
            };

            if path.can_open_valve(&valves) {
                let key = path.derive_next_key();
                let record = opening_records.get(&key);
                if let Some(x) = record {
                    if *x > path.released_pressure {
                        // println!("Better path already open in a previous iteration {i}. Key: ${key}");
                        path.stop(remaining_minutes);
                        paths_to_be_removed.push(*i);
                        continue;
                    }
                }
            }

            // Does not work
            // let valve_max_potential_released_pressure = path.derive_maximum_potential_released_pressure(&valves, remaining_minutes + 1);
            // if valve_max_potential_released_pressure < maximum_potential_released_pressure {
            //     println!("Path {i} is not strong enough");
            //     path.stop(remaining_minutes);
            //     paths_to_be_removed.push(*i);

            //     continue;
            // } else {
            //     maximum_potential_released_pressure = valve_max_potential_released_pressure;
            // }

            if path.derive_maximum_potential_released_pressure(&valves, remaining_minutes + 1)
                < maximum_released_pressure
            {
                // println!("Path {i} is not strong enough");
                path.stop(remaining_minutes);
                paths_to_be_removed.push(*i);

                continue;
            }

            let other_possibilites = path.next_valves_possibilites(&valves);
            for next_valve_id in other_possibilites {
                let mut new_path = path.clone();
                new_path.move_to_new_valve(&next_valve_id);
                paths_to_be_added.push(new_path);
            }

            if path.can_open_valve(&valves) {
                // println!("[Path {i}] - Open valve {}", path.current_valve_id);
                path.open_valve(&valves);
                opening_records.insert(path.derive_key(), path.released_pressure);
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

        println!("\n");
        minutes += 1;
    }

    for p in paths.values() {
        if p.released_pressure > maximum_released_pressure {
            println!("Found one at the end! {}", p.released_pressure);
            maximum_released_pressure = p.released_pressure;
        }
    }

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

    fn derive_key(&self) -> String {
        let mut opened_valves = self
            .open_valves
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>();
        opened_valves.sort();
        return opened_valves.join(" ");
    }
    fn derive_next_key(&self) -> String {
        let mut opened_valves = self
            .open_valves
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>();
        opened_valves.push(&self.current_valve_id);
        opened_valves.sort();
        return opened_valves.join(" ");
    }

    fn derive_maximum_potential_released_pressure(
        &self,
        valves: &HashMap<String, Valve>,
        remaining_minutes: usize,
    ) -> usize {
        if self.stopped {
            return self.released_pressure;
        }

        let mut non_open_valves_rates = valves
            .iter()
            .filter_map(|(v_id, v)| {
                if self.open_valves.contains(v_id) || v.rate == 0 {
                    None
                } else {
                    Some(v.rate)
                }
            })
            .collect::<Vec<usize>>();

        non_open_valves_rates.sort_unstable();

        let mut maximum_potential =
            self.released_pressure + self.released_pressure_rate * remaining_minutes;
        let mut non_open_release_pressure_rate = 0;

        let mut iter_remaining_minutes = remaining_minutes;

        if non_open_valves_rates.len() > 0 {
            let mut i = non_open_valves_rates.len() - 1;

            loop {
                non_open_release_pressure_rate += non_open_valves_rates[i];
                maximum_potential += non_open_release_pressure_rate * 2;
                iter_remaining_minutes = if iter_remaining_minutes > 2 {
                    iter_remaining_minutes - 2
                } else {
                    0
                };
                if i == 0 {
                    break;
                }
                i -= 1;
            }

            maximum_potential += non_open_release_pressure_rate * iter_remaining_minutes;
        }

        maximum_potential
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

        // let non_open_valves = valves.iter().filter(|(v_id, v)| {
        //     !self.open_valves.contains(*v_id) && v.rate > 0
        // }).count();
        // if non_open_valves == 0 {
        //     println!("REMAINING valves to open #{}", non_open_valves);
        // }
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
