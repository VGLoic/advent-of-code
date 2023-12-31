use std::{
    self,
    collections::{HashMap, HashSet},
};

pub fn find_shortest_path(filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;
    let hill_climb = HillClimb::try_from(content.as_str())?;

    let mut paths: HashMap<usize, HillPath> = HashMap::new();

    let mut global_path_index = 0;

    paths.insert(
        global_path_index,
        HillPath::new(hill_climb.starting_position),
    );

    println!(
        "Start: {}",
        hill_climb.hill[hill_climb.starting_position.0][hill_climb.starting_position.1]
    );
    println!(
        "Target: {}",
        hill_climb.hill[hill_climb.target_position.0][hill_climb.target_position.1]
    );

    let mut visited_indices: HashSet<(usize, usize)> = HashSet::new();
    visited_indices.insert(hill_climb.starting_position);

    let mut iteration = 0;
    loop {
        iteration += 1;

        let mut new_paths: Vec<HillPath> = vec![];

        println!("Iteration [{iteration}] - I iterate with {} paths", {
            paths.len()
        });

        if paths.len() == 0 {
            return Err("All created paths have been stopped without finding a solution".into());
        }

        let mut path_indices_to_remove = vec![];

        for (index, path) in paths.iter_mut() {
            // println!("I iterate with path #{index}");

            let possibilities = path
                .derive_possibilities(&hill_climb)
                .into_iter()
                .filter(|p| !visited_indices.contains(p))
                .collect::<Vec<(usize, usize)>>();

            // println!("Path {index} - Possibilities: {:?}", possibilities);

            match possibilities.len() {
                0 => {
                    path_indices_to_remove.push(*index);
                }
                1 => {
                    visited_indices.insert(possibilities[0]);
                    path.visit(possibilities[0], &hill_climb);
                    if path.has_reached_target(&hill_climb) {
                        println!("Path {index} has reached target!");
                        return Ok(path.iteration);
                    }
                }
                l => {
                    for i in 1..l {
                        visited_indices.insert(possibilities[i]);
                        let mut new_path = path.clone();
                        new_path.visit(possibilities[i], &hill_climb);
                        if new_path.has_reached_target(&hill_climb) {
                            println!("Path {index} has reached target!");
                            return Ok(new_path.iteration);
                        }
                        new_paths.push(new_path);
                    }
                    visited_indices.insert(possibilities[0]);
                    path.visit(possibilities[0], &hill_climb);
                    if path.has_reached_target(&hill_climb) {
                        println!("Path {index} has reached target!");
                        return Ok(path.iteration);
                    }
                }
            }
        }

        println!(
            "Iteration [{iteration}] - Removing {} paths",
            path_indices_to_remove.len()
        );
        println!(
            "Iteration [{iteration}] - Adding {} new paths",
            new_paths.len()
        );

        for index in path_indices_to_remove {
            paths.remove(&index);
        }

        for new_path in new_paths {
            global_path_index += 1;
            paths.insert(global_path_index, new_path);
        }
    }
}

pub fn find_shortest_path_from_any_lowest_point(
    filename: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;
    let hill_climb = HillClimb::try_from(content.as_str())?;

    let mut paths: HashMap<usize, HillPath> = HashMap::new();

    let mut global_path_index = 0;

    paths.insert(
        global_path_index,
        HillPath::new(hill_climb.starting_position),
    );

    let mut successful_path_lengths = vec![];

    let mut visited_indices: HashMap<(usize, usize), usize> = HashMap::new();
    visited_indices.insert(hill_climb.starting_position, 0);

    let mut iteration = 0;
    loop {
        iteration += 1;

        let mut new_paths: Vec<HillPath> = vec![];

        println!("Iteration [{iteration}] - I iterate with {} paths", {
            paths.len()
        });

        if paths.len() == 0 {
            return successful_path_lengths
                .into_iter()
                .min()
                .ok_or("Target path has not been reached :(".into());
        }

        let mut path_indices_to_remove = vec![];

        for (index, path) in paths.iter_mut() {
            // println!("I iterate with path #{index}");

            let possibilities = path
                .derive_possibilities(&hill_climb)
                .into_iter()
                .filter(|p| {
                    let existing_iteration_record = visited_indices.get(p);
                    if let Some(existing_iteration) = existing_iteration_record {
                        if *existing_iteration == 0 {
                            return false;
                        }
                        return path.iteration_since_last_low_point < existing_iteration - 1;
                    } else {
                        return true;
                    }
                })
                .collect::<Vec<(usize, usize)>>();

            // println!("Path {index} - Possibilities: {:?}", possibilities);

            match possibilities.len() {
                0 => {
                    path_indices_to_remove.push(*index);
                }
                1 => {
                    path.visit(possibilities[0], &hill_climb);
                    visited_indices.insert(possibilities[0], path.iteration_since_last_low_point);
                    if path.has_reached_target(&hill_climb) {
                        println!("Path {index} has reached target!");
                        successful_path_lengths.push(path.iteration_since_last_low_point);
                        path_indices_to_remove.push(*index);
                    }
                }
                l => {
                    for i in 1..l {
                        let mut new_path = path.clone();
                        new_path.visit(possibilities[i], &hill_climb);
                        visited_indices
                            .insert(possibilities[i], new_path.iteration_since_last_low_point);
                        if new_path.has_reached_target(&hill_climb) {
                            println!("Path {index} has reached target!");
                            successful_path_lengths.push(new_path.iteration_since_last_low_point);
                        } else {
                            new_paths.push(new_path);
                        }
                    }
                    path.visit(possibilities[0], &hill_climb);
                    visited_indices.insert(possibilities[0], path.iteration_since_last_low_point);
                    if path.has_reached_target(&hill_climb) {
                        println!("Path {index} has reached target!");
                        successful_path_lengths.push(path.iteration_since_last_low_point);
                        path_indices_to_remove.push(*index);
                    }
                }
            }
        }

        println!(
            "Iteration [{iteration}] - Removing {} paths",
            path_indices_to_remove.len()
        );
        println!(
            "Iteration [{iteration}] - Adding {} new paths",
            new_paths.len()
        );

        for index in path_indices_to_remove {
            paths.remove(&index);
        }

        for new_path in new_paths {
            global_path_index += 1;
            paths.insert(global_path_index, new_path);
        }
    }
}

#[derive(Debug, Clone)]
struct HillPath {
    iteration: usize,
    iteration_since_last_low_point: usize,
    head: (usize, usize),
}

impl HillPath {
    fn new(start: (usize, usize)) -> Self {
        HillPath {
            head: start,
            iteration: 0,
            iteration_since_last_low_point: 0,
        }
    }
    fn visit(&mut self, p: (usize, usize), hill_climb: &HillClimb) {
        self.head = p;
        self.iteration += 1;
        if hill_climb.hill[p.0][p.1] == 'a' || hill_climb.hill[p.0][p.1] == 'S' {
            self.iteration_since_last_low_point = 0;
        } else {
            self.iteration_since_last_low_point += 1;
        }
    }

    fn has_reached_target(&self, hill_climb: &HillClimb) -> bool {
        hill_climb.hill[self.head.0][self.head.1] == 'E'
    }
}

impl HillPath {
    fn derive_possibilities(&self, hill_climb: &HillClimb) -> Vec<(usize, usize)> {
        let head_value = hill_climb.hill[self.head.0][self.head.1];
        let mut possibilities: Vec<(usize, usize)> = vec![];
        if self.head.1 < hill_climb.x_dim() - 1 {
            possibilities.push((self.head.0, self.head.1 + 1));
        }
        if self.head.1 > 0 {
            possibilities.push((self.head.0, self.head.1 - 1));
        }
        if self.head.0 < hill_climb.y_dim() - 1 {
            possibilities.push((self.head.0 + 1, self.head.1));
        }
        if self.head.0 > 0 {
            possibilities.push((self.head.0 - 1, self.head.1));
        }
        return possibilities
            .into_iter()
            .filter(|p| {
                let p_value = hill_climb.hill[p.0][p.1];
                let digit_p_value = match p_value {
                    'E' => 'z'.to_digit(36).unwrap(),
                    'S' => 'a'.to_digit(36).unwrap(),
                    other => other.to_digit(36).unwrap(),
                };
                let digit_head_value = match head_value {
                    'E' => 'z'.to_digit(36).unwrap(),
                    'S' => 'a'.to_digit(36).unwrap(),
                    other => other.to_digit(36).unwrap(),
                };
                let is_valid = digit_p_value <= digit_head_value + 1;
                // if is_valid {
                //     println!("Allowed: {} [{:?}] to {} [{:?}]", head_value, self.head, p_value, p);
                // } else {
                //     println!("NOT Allowed: {} [{:?}] to {} [{:?}]", head_value, self.head, p_value, p);
                // }
                return is_valid;
            })
            .collect();
    }
}

#[derive(Debug)]
struct HillClimb {
    starting_position: (usize, usize),
    target_position: (usize, usize),
    hill: Vec<Vec<char>>,
}

impl HillClimb {
    fn x_dim(&self) -> usize {
        self.hill[0].len()
    }

    fn y_dim(&self) -> usize {
        self.hill.len()
    }
}

impl TryFrom<&str> for HillClimb {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut hill: Vec<Vec<char>> = vec![];
        let mut starting_position: Option<(usize, usize)> = None;
        let mut target_position: Option<(usize, usize)> = None;
        for line in value.lines() {
            let mut new_row: Vec<char> = vec![];
            let mut j = 0;

            for c in line.chars() {
                match c {
                    'S' => {
                        if starting_position.is_some() {
                            return Err("A second starting position has been found using the character 'S'. This is not supported".into());
                        }
                        starting_position = Some((hill.len(), j));
                    }
                    'E' => {
                        if target_position.is_some() {
                            return Err("A second target position has been found using the character 'S'. This is not supported".into());
                        }
                        target_position = Some((hill.len(), j));
                    }
                    other => {
                        if !c.is_ascii_lowercase() {
                            return Err(format!("Invalid hill character has been found, only character between 'a' and 'z' are supported. Got {}", other).into());
                        }
                    }
                }

                new_row.push(c);
                j += 1;
            }

            if hill.len() > 0 {
                if new_row.len() != hill[0].len() {
                    return Err("Invalid data for the hill construction. Found two lines with two different length".into());
                }
            }

            hill.push(new_row);
        }

        if starting_position.is_none() {
            return Err("A starting position has not been found".into());
        }

        if target_position.is_none() {
            return Err("An ending position has not been found".into());
        }

        Ok(HillClimb {
            starting_position: starting_position.unwrap(),
            target_position: target_position.unwrap(),
            hill,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1_has_right_answer() {
        assert_eq!(
            find_shortest_path("inputs/input-12-example.txt").unwrap(),
            31
        );
    }

    #[test]
    fn part_1_has_right_answer() {
        assert_eq!(find_shortest_path("inputs/input-12.txt").unwrap(), 497);
    }

    #[test]
    fn example_part_2_has_right_answer() {
        assert_eq!(
            find_shortest_path_from_any_lowest_point("inputs/input-12-example.txt").unwrap(),
            29
        );
    }

    #[test]
    fn part_2_has_right_answer() {
        assert_eq!(
            find_shortest_path_from_any_lowest_point("inputs/input-12.txt").unwrap(),
            492
        );
    }
}
