use std::{cmp, collections::HashSet, fs, hash};

pub fn find_start_of_packet_marker_index(
    target_length: usize,
) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("input-06.txt")?;

    if content.len() < target_length {
        return Err(format!("Input from file does not contain enough character to find a marker, expected at least {}, got {}.", target_length, content.len()).into());
    }

    let iteration_str_as_chars = content.chars().collect::<Vec<char>>();
    let mut previous_characters = iteration_str_as_chars[0..target_length - 1].to_vec();

    let mut i = target_length - 1;

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
            }
            None => {
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
    fn target_length_of_4_should_give_expected_answer() {
        assert_eq!(find_start_of_packet_marker_index(4).unwrap(), 1816);
    }

    #[test]
    fn target_length_of_14_should_give_expected_answer() {
        assert_eq!(find_start_of_packet_marker_index(14).unwrap(), 2625);
    }
}
