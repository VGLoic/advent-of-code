use std::{collections::HashSet, fs};

pub fn count_distinct_tail_positions(
    filename: &str,
    knots_number: usize,
) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(filename)?;

    let instructions = content
        .lines()
        .map(|line| Instruction::try_from_raw(line))
        .collect::<Result<Vec<Instruction>, Box<dyn std::error::Error>>>()?;

    let mut tail_positions = HashSet::new();

    let mut rope = Rope::new(knots_number)?;

    tail_positions.insert((rope.tail().x, rope.tail().y));

    for instruction in &instructions {
        println!("Applying instruction: {}", instruction);
        for _ in 0..instruction.value {
            rope.apply_direction(&instruction.direction)?;
            tail_positions.insert((rope.tail().x, rope.tail().y));
        }
        // println!("Rope: {}", rope);
    }

    Ok(tail_positions.len())
}

#[derive(Debug, Clone)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn square_distance(&self, p: &Position) -> usize {
        let d_s = (self.x - p.x) * (self.x - p.x) + (self.y - p.y) * (self.y - p.y);
        d_s.try_into().unwrap()
    }
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Position>,
}

impl Rope {
    fn new(knots_number: usize) -> Result<Self, Box<dyn std::error::Error>> {
        if knots_number < 2 {
            return Err("Unable to create a rope with less than two knots".into());
        }
        Ok(Rope {
            knots: vec![Position { x: 0, y: 0 }; knots_number],
        })
    }

    fn apply_direction(
        &mut self,
        direction: &Direction,
    ) -> Result<&mut Self, Box<dyn std::error::Error>> {
        match direction {
            Direction::Up => self.knots[0].y += 1,
            Direction::Right => self.knots[0].x += 1,
            Direction::Down => self.knots[0].y -= 1,
            Direction::Left => self.knots[0].x -= 1,
        };
        for i in 1..self.knots.len() {
            let previous_knot = &self.knots[i - 1];
            let knot = &self.knots[i];
            let d_s = previous_knot.square_distance(knot);
            match d_s {
                0 => {}
                1 => {}
                2 => {}
                4 => {
                    let y_diff = previous_knot.y - knot.y;
                    if y_diff.is_positive() {
                        self.knots[i].y += 1;
                        continue;
                    }
                    if y_diff.is_negative() {
                        self.knots[i].y -= 1;
                        continue;
                    }

                    let x_diff = previous_knot.x - knot.x;
                    if x_diff.is_positive() {
                        self.knots[i].x += 1;
                        continue;
                    }
                    if x_diff.is_negative() {
                        self.knots[i].x -= 1;
                        continue;
                    }
                }
                5 | 8 => {
                    let y_diff = previous_knot.y - knot.y;
                    let y_shift = if y_diff.is_positive() {
                        1
                    } else if y_diff == 0 {
                        0
                    } else {
                        -1
                    };
                    let x_diff = previous_knot.x - knot.x;
                    let x_shift = if x_diff.is_positive() {
                        1
                    } else if x_diff == 0 {
                        0
                    } else {
                        -1
                    };
                    self.knots[i].x += x_shift;
                    self.knots[i].y += y_shift;
                }
                other => {
                    return Err(format!(
                        "Unexpected distance between knots {} and knots {}, got {}",
                        i - 1,
                        i,
                        other
                    )
                    .into())
                }
            };
        }
        Ok(self)
    }
    fn tail(&self) -> &Position {
        &self.knots[self.knots.len() - 1]
    }
}

impl std::fmt::Display for Rope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut coordinates = vec![];
        for knot in &self.knots {
            coordinates.push(knot.x);
            coordinates.push(knot.y);
        }
        let max_coordinate: usize = coordinates
            .iter()
            .map(|x| x.abs())
            .max()
            .unwrap()
            .try_into()
            .unwrap();

        let mut grid = vec![vec!['.'; 2 * max_coordinate + 1]; 2 * max_coordinate + 1];

        for i in 1..self.knots.len() {
            let x = (self.knots[i].x + max_coordinate as isize) as usize;
            let y = (self.knots[i].y + max_coordinate as isize) as usize;
            if grid[y][x] == '.' {
                grid[y][x] = char::from_digit(i as u32, 10).unwrap();
            }
        }

        let head_x = (self.knots[0].x + max_coordinate as isize) as usize;
        let head_y = (self.knots[0].y + max_coordinate as isize) as usize;
        grid[head_y][head_x] = 'H';

        if grid[max_coordinate][max_coordinate] == '.' {
            grid[max_coordinate][max_coordinate] = 's';
        }

        let displayed = grid
            .iter()
            .rev()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "\n{}", displayed)
    }
}

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug)]
struct Instruction {
    value: usize,
    direction: Direction,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let displayed = match self.direction {
            Direction::Right => format!("Direction: Right, value: {}", self.value),
            Direction::Down => format!("Direction: Down, value: {}", self.value),
            Direction::Left => format!("Direction: Left, value: {}", self.value),
            Direction::Up => format!("Direction: Up, value: {}", self.value),
        };
        write!(f, "{}", displayed)
    }
}

impl Instruction {
    fn try_from_raw(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let elements: Vec<&str> = content.split(" ").collect();
        if elements.len() != 2 {
            return Err(format!(
                "Unable to parse content, expected two elements separated by whitespace, got {}",
                content
            )
            .into());
        }

        let instruction_value: usize = elements[1].parse()?;

        let instruction_direction_str = elements[0];
        let direction = match instruction_direction_str {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            other => {
                return Err(format!(
                    "Unexpected instruction type, expected `R`, `L`, `U`, `D`, got {}",
                    other
                )
                .into())
            }
        };
        Ok(Instruction {
            value: instruction_value,
            direction,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1_has_right_answer() {
        assert_eq!(
            count_distinct_tail_positions("inputs/input-09-example.txt", 2).unwrap(),
            13
        );
    }

    #[test]
    fn part_1_has_right_answer() {
        assert_eq!(
            count_distinct_tail_positions("inputs/input-09.txt", 2).unwrap(),
            5930
        );
    }

    #[test]
    fn example_part_2_has_right_answer() {
        assert_eq!(
            count_distinct_tail_positions("inputs/input-09-example-part-2.txt", 10).unwrap(),
            36
        );
    }

    #[test]
    fn part_2_has_right_answer() {
        assert_eq!(
            count_distinct_tail_positions("inputs/input-09.txt", 10).unwrap(),
            2443
        );
    }
}
