pub mod first_part {
    use super::*;
    use std::fs;

    pub fn compute_priorities_sum(filename: &str) -> Result<u32, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(filename)?;
        let mut total = 0;
        for line in contents.lines() {
            let (left_compartment, right_compartment) = parse_line_into_compartments(line)?;
            let common_item = find_common_item(left_compartment, right_compartment)
                .ok_or("Unable to find the common item")?;
            let priority = item_to_priority(common_item)?;
            total += priority;
        }
        return Ok(total);
    }

    fn find_common_item(str_a: &str, str_b: &str) -> Option<char> {
        let common_char_opt = str_a.chars().find(|&c| str_b.contains(c));

        return common_char_opt;
    }

    fn parse_line_into_compartments(
        line: &str,
    ) -> Result<(&str, &str), Box<dyn std::error::Error>> {
        let split_index = line.len() / 2;
        let (a, b) = line.split_at(split_index);
        if a.len() != b.len() {
            return Err(format!(
                "Invalid split, got two parts with different lengths, {} and {}",
                a, b
            )
            .into());
        }
        return Ok((a, b));
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn example_part_1_has_right_answer() {
            assert_eq!(
                compute_priorities_sum("inputs/input-03-example.txt").unwrap(),
                157
            );
        }

        #[test]
        fn part_1_has_right_answer() {
            assert_eq!(compute_priorities_sum("inputs/input-03.txt").unwrap(), 7848);
        }
    }
}

pub mod second_part {
    use super::*;
    use std::fs;

    pub fn compute_priorities_sum(filename: &str) -> Result<u32, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(filename)?;
        let mut total = 0;
        let mut i = 0;
        let lines: Vec<_> = contents.lines().collect();
        let number_of_groups = lines.len() / 3;
        while i < number_of_groups {
            let common_char = lines[3 * i]
                .chars()
                .find(|&c| {
                    return lines[3 * i + 1].contains(c) && lines[3 * i + 2].contains(c);
                })
                .ok_or("Unable to find common item in the group")?;
            let priority = item_to_priority(common_char)?;
            total += priority;
            i += 1;
        }
        return Ok(total);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn example_part_2_has_right_answer() {
            assert_eq!(
                compute_priorities_sum("inputs/input-03-example.txt").unwrap(),
                70
            );
        }

        #[test]
        fn part_2_has_right_answer() {
            assert_eq!(compute_priorities_sum("inputs/input-03.txt").unwrap(), 2616);
        }
    }
}

fn item_to_priority(c: char) -> Result<u32, Box<dyn std::error::Error>> {
    let a: u32 = c
        .try_into()
        .or(Err(format!("Unable to convert the char {} to a `u32`", c)))?;
    let number_in_alphabet = 26;
    if a >= 64 && a <= 64 + number_in_alphabet {
        return Ok(a - 64 + number_in_alphabet);
    }
    if a >= 97 && a <= 97 + number_in_alphabet {
        return Ok(a - 96);
    }
    return Err(format!("Conversion of char is in an unmanaged range, expected between {} and {}, or beween {} and {}, got {}", 64, 64 + number_in_alphabet, 97, 97 + number_in_alphabet, a).into());
}
