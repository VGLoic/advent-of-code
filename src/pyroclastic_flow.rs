use std;

pub fn find_tower_height(filename: &str, number_of_rocks: usize) -> Result<usize, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;

    let mut jets = content.chars().filter_map(|c| match Jet::try_from(&c) {
        Ok(d) => Some(d),
        Err(e) => {
            println!(
                "Error while parsing the jet direction: {}. This jet is ignored",
                e
            );
            None
        }
    }).cycle();

    let rocks = define_rocks();

    let mut cave = Cave::new();

    for fallen_rock_count in 0..number_of_rocks {
        let rock_type = &rocks[fallen_rock_count % rocks.len()];
        let mut rock = FallingRock::new(cave.height() + 3, rock_type);

        println!("{}\n", rock.rock_type);

        if let Some(jet) = jets.next() {
            rock.apply_jet(&jet, &cave);
        }
        while cave.can_rock_fall(&rock) {
            rock.fall();
            if let Some(jet) = jets.next() {
                rock.apply_jet(&jet, &cave);
            }
        }

        cave.incorporate_rock(&rock);

        if cave.structure.iter().any(|r| r.iter().all(|x| x == &Element::Air)) {
            panic!("SOUCIS!!!!!!")
        }

        // println!("\n\n")
    }

    println!("{}", cave);

    Ok(cave.height() - 1)
}

struct Cave {
    structure: Vec<Vec<Element>>,
    width: usize,
}

impl Cave {
    fn print_with_falling_rock(&self, rock: &FallingRock) {
        if rock.bottom_left_position.y + rock.rock_type.height() > self.height() {
            let number_of_air_before_the_rock = rock.bottom_left_position.x;
            let number_of_air_after_the_rock = self.width - (rock.bottom_left_position.x + rock.rock_type.width());
            for i in 0..(rock.bottom_left_position.y + rock.rock_type.height() - self.height()) {
                if i >= rock.rock_type.height() {
                    println!("|.......|");
                } else {
                    let mut displayed_row = "|".to_owned();
                    for _ in 0..number_of_air_before_the_rock {
                        displayed_row += ".";
                    }
                    let rock_row_str = rock.rock_type.structure[i].iter().fold("".to_owned(), |a, e| {
                        let c = match e {
                            Element::Air => '.',
                            Element::Rock => '@'
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
        }
    }

    fn height(&self) -> usize {
        self.structure.len()
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
            //check, ok but change for clarity
            let below_cave_element_y_coordinate = rock.bottom_left_position.y + i - 1;
            if below_cave_element_y_coordinate > cave_height - 1 {
                continue;
            }
            for j in 0..rock.rock_type.width() {
                let below_cave_element_x_coordinate = rock.bottom_left_position.x + j;
                if rock_row[j] == Element::Rock
                    && self.structure[below_cave_element_y_coordinate]
                        [below_cave_element_x_coordinate]
                        == Element::Rock
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
                    let y_coordinate = r.bottom_left_position.y + i;
                    let x_coordinate = r.bottom_left_position.x + j;
                    if self.structure[y_coordinate][x_coordinate] == Element::Rock {
                        panic!("OVERLAP: {i} {j}");
                    }
                    self.structure[y_coordinate][x_coordinate] = Element::Rock;
                }
            }
        }
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
                    let left_most_rock_row_x_coordinate = self.bottom_left_position.x + self.rock_type.left_most_rock_index(i);
                    if cave.structure[left_most_rock_y_coordinate][left_most_rock_row_x_coordinate - 1]
                        == Element::Rock
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
                // cave width: 7, cave width - 1 = 6
                // coord: 6 -> nop
                // coord: 5 -> yes
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
                    let right_most_rock_row_x_coordinate = self.bottom_left_position.x + self.rock_type.right_most_rock_index(i);
                    if cave.structure[right_most_rock_y_coordinate]
                        [right_most_rock_row_x_coordinate + 1]
                        == Element::Rock
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
        panic!("Impossible case of no rock in row");
    }
    
    fn right_most_rock_index(&self, row_index: usize) -> usize {
        for i in (0..self.width()).rev() {
            let e = &self.structure[row_index][i];
            if e == &Element::Rock {
                return i;
            }
        }
        panic!("Impossible case of no rock in row");
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

impl std::fmt::Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut displayed_string = "".to_owned();
        for i in (1..self.structure.len()).rev() {
            displayed_string += "|";
            displayed_string += self.structure[i]
                .iter()
                .fold("".to_owned(), |a, e| format!("{a}{e}"))
                .as_str();
            displayed_string += "|\n";
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
}
