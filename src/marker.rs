use std::{cmp, collections::HashSet, fs, hash};

pub fn find_start_of_packet_marker_index(
    filename: &str,
    target_length: usize,
) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(filename)?;

    if content.len() < target_length {
        return Err(format!("inputs/input from file does not contain enough character to find a marker, expected at least {}, got {}.", target_length, content.len()).into());
    }

    let iteration_str_as_chars = content.chars().collect::<Vec<char>>();

    let mut previous_characters = iteration_str_as_chars[0..target_length - 1].to_vec();
    let mut i = target_length - 1;
    while !has_unique_elements(&previous_characters) {
        if i >= iteration_str_as_chars.len() {
            return Err("Unable to find a marker :(".into());
        }

        i += 1;
        previous_characters = iteration_str_as_chars[i + 1 - target_length..i].to_vec();
    }

    while i < iteration_str_as_chars.len() {
        let c = iteration_str_as_chars[i];

        let existing_position = previous_characters.iter().rev().position(|x| x == &c);

        match existing_position {
            Some(j) => {
                let mut jump_size = target_length - j - 1;
                let mut new_previous_characters = iteration_str_as_chars
                    [i + 1 - target_length + jump_size..i + jump_size]
                    .to_vec();
                while !has_unique_elements(&new_previous_characters) {
                    jump_size += 1;
                    new_previous_characters = iteration_str_as_chars
                        [i + 1 - target_length + jump_size..i + jump_size]
                        .to_vec();
                }
                previous_characters = iteration_str_as_chars
                    [i + 1 - target_length + jump_size..i + jump_size]
                    .to_vec();
                i += jump_size;
                println!("Jumping: {}", i);
            }
            None => {
                println!("{:?}", previous_characters);
                return Ok(i + 1);
            }
        }
    }

    return Err("Unable to find a marker :(".into());
}

fn has_unique_elements<T>(iterable: T) -> bool
where
    T: IntoIterator,
    T::Item: cmp::Eq + hash::Hash,
{
    let mut set = HashSet::new();
    return iterable.into_iter().all(|x| set.insert(x));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_target_length_of_4_should_give_expected_answer() {
        assert_eq!(
            find_start_of_packet_marker_index("inputs/input-06-example.txt", 4).unwrap(),
            7
        );
    }

    #[test]
    fn target_length_of_4_should_give_expected_answer() {
        assert_eq!(
            find_start_of_packet_marker_index("inputs/input-06.txt", 4).unwrap(),
            1816
        );
    }

    #[test]
    fn example_target_length_of_14_should_give_expected_answer() {
        assert_eq!(
            find_start_of_packet_marker_index("inputs/input-06-example.txt", 14).unwrap(),
            19
        );
    }

    #[test]
    fn target_length_of_14_should_give_expected_answer() {
        assert_eq!(
            find_start_of_packet_marker_index("inputs/input-06.txt", 14).unwrap(),
            2625
        );
    }
}
