use std::{self, collections::HashMap};

pub fn find_tower_height(
    filename: &str,
    number_of_rocks: usize,
) -> Result<usize, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;

    let mut jets = content
        .chars()
        .filter_map(|c| match Jet::try_from(&c) {
            Ok(d) => Some(d),
            Err(_) => None,
        })
        .cycle();

    let rocks = define_rocks();

    let mut cave = Cave::new();

    let mut jet_index = 0;
    let mut surface_depth = cave.rocky_surface_depth();

    let mut repetitions: HashMap<(usize, usize, usize), Vec<(usize, usize, usize)>> =
        HashMap::new();

    let mut fallen_rock_count = 0;

    while fallen_rock_count < number_of_rocks {
        if fallen_rock_count % 100_000 == 0 {
            println!("Rock #{fallen_rock_count}");
        }
        let rock_type = &rocks[fallen_rock_count % rocks.len()];
        let mut rock = FallingRock::new(cave.height() + 3, rock_type);

        if let Some(jet) = jets.next() {
            jet_index += 1;
            rock.apply_jet(&jet, &cave);
        }
        while cave.can_rock_fall(&rock) {
            rock.fall();
            if let Some(jet) = jets.next() {
                jet_index += 1;
                rock.apply_jet(&jet, &cave);
            }
        }

        cave.incorporate_rock(&rock);

        let new_surface_depth = cave.rocky_surface_depth();
        if new_surface_depth < surface_depth {
            let rows_to_truncate =
                cave.height() - cave.truncated_height - 1 - (new_surface_depth + 1);
            if rows_to_truncate > 0 {
                cave.truncate(rows_to_truncate);
            }
            let rock_index = fallen_rock_count % rocks.len();
            let repetition_tuple = (surface_depth, new_surface_depth, rock_index);
            if let Some(r) = repetitions.get_mut(&repetition_tuple) {
                if r.len() == 1 {
                    r.push((fallen_rock_count, jet_index, cave.height()));
                } else {
                    let rock_diff = r[1].0 - r[0].0;
                    let jet_diff = r[1].1 - r[0].1;
                    let h_diff = r[1].2 - r[0].2;
                    let repetition_detected = fallen_rock_count - r[1].0 == rock_diff
                        && jet_index - r[1].1 == jet_diff
                        && cave.height() - r[1].2 == h_diff;
                    if repetition_detected {
                        let q = (number_of_rocks - fallen_rock_count) / rock_diff;
                        fallen_rock_count += q * rock_diff;
                        jet_index += q * jet_diff;
                        cave.truncated_height += q * h_diff;
                    }
                }
            } else {
                repetitions.insert(
                    repetition_tuple,
                    vec![(fallen_rock_count, jet_index, cave.height())],
                );
            }
        }
        surface_depth = new_surface_depth;

        fallen_rock_count += 1;
    }

    // println!("{}", cave);

    Ok(cave.height() - 1)
}

struct Cave {
    structure: Vec<Vec<Element>>,
    truncated_height: usize,
    width: usize,
}

impl Cave {
    fn rocky_surface_depth(&self) -> usize {
        let mut already_explored_height = 0;
        loop {
            // Find left rock below the already explored height
            while !self.is_rock(0, self.height() - 1 - already_explored_height) {
                already_explored_height += 1;
            }
            // If ground is found, return
            if already_explored_height == self.height() - 1 {
                return self.height() - 1;
            }
            // println!("Trying to find a rocky path with height {} at coordinate {}", self.height(), self.height() - 1 - already_explored_height);
            // Check if we there are rocks in contact for the full line
            let mut rocky_path_finder =
                RockyPathFinder::new(self.height() - 1 - already_explored_height, self);
            let mut depth = already_explored_height;
            while rocky_path_finder.can_walk(self) {
                rocky_path_finder.walk(self);
                let current_depth = self.height() - 1 - rocky_path_finder.current_position.y;
                if current_depth > depth {
                    depth = current_depth;
                }
            }
            if rocky_path_finder.current_position.x == self.width - 1 {
                return depth;
            }
            if rocky_path_finder.current_position.x == 0 {
                already_explored_height += 1;
            }
        }
    }

    #[allow(dead_code)]
    // Only handle case of rocks above the height of the cave
    // TODO: handle rock in the cave
    fn print_with_falling_rock(&self, rock: &FallingRock) {
        if rock.bottom_left_position.y + rock.rock_type.height() > self.height() {
            let number_of_air_before_the_rock = rock.bottom_left_position.x;
            let number_of_air_after_the_rock =
                self.width - (rock.bottom_left_position.x + rock.rock_type.width());
            for i in 0..(rock.bottom_left_position.y + rock.rock_type.height() - self.height()) {
                if i >= rock.rock_type.height() {
                    println!("|.......|");
                } else {
                    let mut displayed_row = "|".to_owned();
                    for _ in 0..number_of_air_before_the_rock {
                        displayed_row += ".";
                    }
                    let rock_row_str =
                        rock.rock_type.structure[i]
                            .iter()
                            .fold("".to_owned(), |a, e| {
                                let c = match e {
                                    Element::Air => '.',
                                    Element::Rock => '@',
                                };
                                format!("{a}{c}")
                            });
                    displayed_row += rock_row_str.as_str();
                    for _ in 0..number_of_air_after_the_rock {
                        displayed_row += ".";
                    }
                    displayed_row += "|";
                    println!("{}", displayed_row);
                }
            }
        }
        println!("{}", self);
    }

    fn new() -> Self {
        Cave {
            width: 7,
            structure: vec![vec![Element::Rock; 7]],
            truncated_height: 0,
        }
    }

    fn get_element(&self, x_coordinate: usize, y_coordinate: usize) -> &Element {
        &self.structure[y_coordinate - self.truncated_height][x_coordinate]
    }

    fn is_rock(&self, x_coordinate: usize, y_coordinate: usize) -> bool {
        self.structure[y_coordinate - self.truncated_height][x_coordinate] == Element::Rock
    }

    fn is_out_of_bound(&self, x_coordinate: usize, y_coordinate: usize) -> bool {
        if x_coordinate > self.width - 1 {
            return true;
        }
        y_coordinate > self.height() - 1
    }

    fn height(&self) -> usize {
        self.structure.len() + self.truncated_height
    }

    fn can_rock_fall(&self, rock: &FallingRock) -> bool {
        let cave_height = self.height();
        // If the rock is higher than the current height of the cave, it can fall in amy case
        if rock.bottom_left_position.y > cave_height {
            return true;
        }
        // For each row of the row, starting from bottom, we check if there are two rocks in contact
        for i in 0..rock.rock_type.height() {
            // We iterate from bottom left to top right
            let rock_row_index = rock.rock_type.height() - i - 1;
            let rock_row = &rock.rock_type.structure[rock_row_index];
            let below_cave_element_y_coordinate = rock.bottom_left_position.y + i - 1;
            if below_cave_element_y_coordinate > cave_height - 1 {
                continue;
            }
            for j in 0..rock.rock_type.width() {
                let below_cave_element_x_coordinate = rock.bottom_left_position.x + j;
                if rock_row[j] == Element::Rock
                    && self.get_element(
                        below_cave_element_x_coordinate,
                        below_cave_element_y_coordinate,
                    ) == &Element::Rock
                {
                    return false;
                }
            }
        }
        return true;
    }

    fn incorporate_rock(&mut self, r: &FallingRock) {
        if r.bottom_left_position.y + r.rock_type.height() > self.height() {
            for _ in 0..(r.bottom_left_position.y + r.rock_type.height() - self.height()) {
                self.structure.push(vec![Element::Air; self.width])
            }
        }

        for i in 0..r.rock_type.height() {
            for j in 0..r.rock_type.width() {
                // We iterate from bottom left to top right
                if r.rock_type.structure[r.rock_type.height() - 1 - i][j] == Element::Rock {
                    let y_coordinate = r.bottom_left_position.y + i - self.truncated_height;
                    let x_coordinate = r.bottom_left_position.x + j;
                    if self.structure[y_coordinate][x_coordinate] == Element::Rock {
                        panic!("OVERLAP: {i} {j}");
                    }
                    self.structure[y_coordinate][x_coordinate] = Element::Rock;
                }
            }
        }
    }

    fn truncate(&mut self, number_of_truncated_rows: usize) {
        self.truncated_height += number_of_truncated_rows;
        self.structure.drain(0..number_of_truncated_rows);
    }
}

struct FallingRock<'a> {
    bottom_left_position: Position,
    rock_type: &'a Rock,
}

impl<'a> FallingRock<'a> {
    fn new(starting_height: usize, rock_type: &'a Rock) -> Self {
        FallingRock {
            bottom_left_position: Position {
                x: 2,
                y: starting_height,
            },
            rock_type,
        }
    }

    fn fall(&mut self) {
        self.bottom_left_position.y -= 1;
    }

    fn apply_jet(&mut self, jet: &Jet, cave: &Cave) {
        match jet {
            Jet::Left => {
                if self.bottom_left_position.x == 0 {
                    // Out of bound
                    return;
                }
                // We check that each row is not blocked by an element at the left
                for i in 0..self.rock_type.height() {
                    // We iterate from top to bottom
                    let left_most_rock_y_coordinate =
                        self.bottom_left_position.y + self.rock_type.height() - 1 - i;
                    if left_most_rock_y_coordinate > cave.height() - 1 {
                        continue;
                    }
                    let left_most_rock_row_x_coordinate =
                        self.bottom_left_position.x + self.rock_type.left_most_rock_index(i);
                    if cave.get_element(
                        left_most_rock_row_x_coordinate - 1,
                        left_most_rock_y_coordinate,
                    ) == &Element::Rock
                    {
                        // Rock found at the left, we can not move left
                        return;
                    }
                }
                self.bottom_left_position.x -= 1;
            }
            Jet::Right => {
                let right_most_rock_x_coordinate =
                    self.bottom_left_position.x + self.rock_type.width() - 1;
                if right_most_rock_x_coordinate >= cave.width - 1 {
                    // Out of bound
                    return;
                }
                // We check that each row is not blocked by an element at the right
                for i in 0..self.rock_type.height() {
                    // We iterate from top to bottom
                    let right_most_rock_y_coordinate =
                        self.bottom_left_position.y + self.rock_type.height() - 1 - i;
                    if right_most_rock_y_coordinate > cave.height() - 1 {
                        continue;
                    }
                    let right_most_rock_row_x_coordinate =
                        self.bottom_left_position.x + self.rock_type.right_most_rock_index(i);
                    if cave.get_element(
                        right_most_rock_row_x_coordinate + 1,
                        right_most_rock_y_coordinate,
                    ) == &Element::Rock
                    {
                        // Rock found at the right, we can not move right
                        return;
                    }
                }
                self.bottom_left_position.x += 1;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Rock {
    structure: Vec<Vec<Element>>,
}

impl Rock {
    fn width(&self) -> usize {
        self.structure[0].len()
    }
    fn height(&self) -> usize {
        self.structure.len()
    }

    fn left_most_rock_index(&self, row_index: usize) -> usize {
        for i in 0..self.width() {
            let e = &self.structure[row_index][i];
            if e == &Element::Rock {
                return i;
            }
        }
        panic!("Unreachable: Impossible case of no rock in row");
    }

    fn right_most_rock_index(&self, row_index: usize) -> usize {
        for i in (0..self.width()).rev() {
            let e = &self.structure[row_index][i];
            if e == &Element::Rock {
                return i;
            }
        }
        panic!("Unreachable: Impossible case of no rock in row");
    }
}

impl TryFrom<Vec<Vec<Element>>> for Rock {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: Vec<Vec<Element>>) -> Result<Self, Self::Error> {
        if value.len() == 0 {
            return Err("Got zero rows, empty rock is not allowed".into());
        }
        let row_size = value[0].len();
        if value.iter().any(|r| r.len() != row_size) {
            return Err("Rock structure must have rows of same size".into());
        }
        // TODO: add also check on column
        if value.iter().any(|r| r.iter().all(|e| e == &Element::Air)) {
            return Err("Rock can not have row filled with air only".into());
        }
        if row_size == 0 {
            return Err("Gor zero columns, empty rock is not allowed".into());
        }
        Ok(Rock { structure: value })
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Element {
    Air,
    Rock,
}

struct RockyPathFinder {
    previous: Vec<Position>,
    current_position: Position,
    remaining_options: HashMap<Position, Vec<Position>>,
}

impl RockyPathFinder {
    fn new(y: usize, cave: &Cave) -> Self {
        let p = Position { x: 0, y };
        let possibilities = RockyPathFinder::next_possibilities(&p, cave, None);
        let mut options_map = HashMap::new();
        options_map.insert(p.clone(), possibilities);
        RockyPathFinder {
            previous: vec![],
            current_position: p,
            remaining_options: options_map,
        }
    }

    fn go_back(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let previous_position = self.previous.pop();
        match previous_position {
            None => return Err("Unable to go back as there are no previous position".into()),
            Some(p) => {
                self.current_position = p;
            }
        };
        Ok(())
    }

    fn next_possibilities(
        p: &Position,
        cave: &Cave,
        previous_position: Option<&Position>,
    ) -> Vec<Position> {
        if cave.is_out_of_bound(p.x, p.y) {
            return vec![];
        }

        if p.x == cave.width - 1 {
            return vec![];
        }

        let mut possibilities = vec![
            Position { x: p.x, y: p.y + 1 },
            Position {
                x: p.x + 1,
                y: p.y + 1,
            },
            Position { x: p.x + 1, y: p.y },
        ];
        if p.y > 1 {
            possibilities.push(Position {
                x: p.x + 1,
                y: p.y - 1,
            });
            if p.x > 0 {
                possibilities.push(Position { x: p.x, y: p.y - 1 });
            }
        }
        possibilities.reverse();

        possibilities
            .into_iter()
            .filter(|next_p| {
                if let Some(previous_p) = previous_position {
                    if previous_p == next_p {
                        return false;
                    }
                }
                !cave.is_out_of_bound(next_p.x, next_p.y) && cave.is_rock(next_p.x, next_p.y)
            })
            .collect::<Vec<Position>>()
    }

    fn walk(&mut self, cave: &Cave) {
        let options = self
            .remaining_options
            .get_mut(&self.current_position)
            .ok_or("Unreachable: Unable to found the remaining options for a position")
            .unwrap();
        // println!("Walking from {:?} with #{} possibilities", self.current_position, options.len());
        let next_position = options.pop();
        match next_position {
            None => {
                // println!("I go back");
                self.go_back().unwrap();
            }
            Some(next_p) => {
                let next_possibilities = RockyPathFinder::next_possibilities(
                    &next_p,
                    cave,
                    Some(&self.current_position),
                );

                if !self.remaining_options.contains_key(&next_p) {
                    self.remaining_options
                        .insert(next_p.clone(), next_possibilities);
                }
                self.previous.push(self.current_position.clone());
                self.current_position = next_p;
            }
        }
    }

    fn can_walk(&self, cave: &Cave) -> bool {
        if self.current_position.x == cave.width - 1 {
            return false;
        }
        let options = self
            .remaining_options
            .get(&self.current_position)
            .ok_or("Unreachable: Unable to found the remaining options for a position")
            .unwrap();
        options.len() > 0 || self.previous.len() > 0
    }
}

impl std::fmt::Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut displayed_string = "".to_owned();
        for i in (1..self.height() - self.truncated_height).rev() {
            displayed_string += "|";
            displayed_string += self.structure[i]
                .iter()
                .fold("".to_owned(), |a, e| format!("{a}{e}"))
                .as_str();
            displayed_string += "|\n";
        }
        if self.truncated_height > 0 {
            displayed_string += "|_______|\n";
        }
        displayed_string += "+";
        for _ in 0..self.width {
            displayed_string += "-";
        }
        displayed_string += "+";
        write!(f, "{}", displayed_string)
    }
}

impl std::fmt::Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut displayed_string = "".to_owned();
        for i in 0..self.structure.len() {
            let row = &self.structure[i];
            let r_string = row.iter().fold("".to_owned(), |a, e| format!("{a}{e}"));
            displayed_string += r_string.as_str();
            if i != self.structure.len() - 1 {
                displayed_string += "\n";
            }
        }
        write!(f, "{}", displayed_string)
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = match self {
            Element::Air => '.',
            Element::Rock => '#',
        };
        write!(f, "{}", a)
    }
}

#[derive(Debug)]
enum Jet {
    Left,
    Right,
}

impl TryFrom<&char> for Jet {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Jet::Left),
            '>' => Ok(Jet::Right),
            other => Err(format!(
                "Invalid provided value for jet, expected '<' or '>', got {}",
                other
            )
            .into()),
        }
    }
}

fn define_rocks() -> Vec<Rock> {
    vec![
        // ####
        Rock::try_from(vec![vec![
            Element::Rock,
            Element::Rock,
            Element::Rock,
            Element::Rock,
        ]])
        .unwrap(),
        // .#.
        // ###
        // .#.
        Rock::try_from(vec![
            vec![Element::Air, Element::Rock, Element::Air],
            vec![Element::Rock, Element::Rock, Element::Rock],
            vec![Element::Air, Element::Rock, Element::Air],
        ])
        .unwrap(),
        // ..#
        // ..#
        // ###
        Rock::try_from(vec![
            vec![Element::Air, Element::Air, Element::Rock],
            vec![Element::Air, Element::Air, Element::Rock],
            vec![Element::Rock, Element::Rock, Element::Rock],
        ])
        .unwrap(),
        // #
        // #
        // #
        // #
        Rock::try_from(vec![
            vec![Element::Rock],
            vec![Element::Rock],
            vec![Element::Rock],
            vec![Element::Rock],
        ])
        .unwrap(),
        // ##
        // ##
        Rock::try_from(vec![
            vec![Element::Rock, Element::Rock],
            vec![Element::Rock, Element::Rock],
        ])
        .unwrap(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1_should_give_expected_result() {
        assert_eq!(
            find_tower_height("inputs/input-17-example.txt", 2_022).unwrap(),
            3068
        );
    }

    #[test]
    fn part_1_should_give_expected_result() {
        assert_eq!(
            find_tower_height("inputs/input-17.txt", 2_022).unwrap(),
            3111
        );
    }

    #[test]
    fn example_part_2_should_give_expected_result() {
        assert_eq!(
            find_tower_height("inputs/input-17-example.txt", 1_000_000_000_000).unwrap(),
            1514285714288
        );
    }

    #[test]
    fn part_2_should_give_expected_result() {
        assert_eq!(
            find_tower_height("inputs/input-17.txt", 1_000_000_000_000).unwrap(),
            1526744186042
        );
    }
}
