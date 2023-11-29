use std;

pub fn find_shortest_path(filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;
    let hill_climb = HillClimb::try_from(content.as_str())?;

    println!("{:?}", hill_climb);

    Ok(3)
}

#[derive(Debug)]
struct HillClimb {
    starting_position: (usize, usize),
    target_position: (usize, usize),
    hill: Vec<Vec<char>>
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
                    },
                    'E' => {
                        if target_position.is_some() {
                            return Err("A second target position has been found using the character 'S'. This is not supported".into());
                        }
                        target_position = Some((hill.len(), j));
                    },
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

        Ok(HillClimb { starting_position: starting_position.unwrap(), target_position: target_position.unwrap(), hill })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1_has_right_answer() {
        assert_eq!(
            find_shortest_path("inputs/input-11.example.txt").unwrap(),
            31
        );
    }
}
