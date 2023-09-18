use std::{
    cmp,
    collections::{HashSet, VecDeque},
    fs, hash,
};

pub fn find_start_of_packet_marker_index() -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("input-06.txt")?;

    let target_length = 4;

    if content.len() < target_length {
        return Err(format!("Input from file does not contain enough character to find a marker, expected at least {}, got {}.", target_length, content.len()).into());
    }

    let mut iteration = 0;

    let iteration_str_as_chars = content.chars().collect::<Vec<char>>();
    let mut previous_characters = iteration_str_as_chars[0..target_length - 1].to_vec();

    let mut i = target_length - 1;

    while i < iteration_str_as_chars.len() {
        iteration += 1;

        let c = iteration_str_as_chars[i];

        let existing_position = previous_characters.iter().rev().position(|x| x == &c);

        match existing_position {
            Some(j) => {
                let mut jump_size = target_length - j - 1;
                // println!("Not uniq! {:?} {}", previous_characters, c);
                let mut new_previous_characters = iteration_str_as_chars[i + 1 - target_length + jump_size..i + jump_size].to_vec();
                while !has_unique_elements(&new_previous_characters) {
                    // println!("Has duplicate: {:?}", new_previous_characters);
                    jump_size += 1;
                    new_previous_characters = iteration_str_as_chars[i + 1 - target_length + jump_size..i + jump_size].to_vec();
                }
                previous_characters = iteration_str_as_chars[i + 1 - target_length + jump_size..i + jump_size].to_vec();
                // println!("Jump of {}", jump_size);
                i += jump_size;
            },
            None => {
                println!("Iteration {}", iteration);
                return Ok(i + target_length);
            }
        }
    }

    return Err("Unable to find a marker :(".into());
}
pub fn find_start_of_message_marker_index() -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("input-06.txt")?;

    let target_length = 4;

    if content.len() < target_length {
        return Err(format!("Input from file does not contain enough character to find a marker, expected at least {}, got {}.", target_length, content.len()).into());
    }

    let mut iteration = 0;

    let (init_str, iteration_string) = content.split_at(target_length - 1);

    let mut previous_characters = VecDeque::from(init_str.chars().collect::<Vec<char>>());
    
    let iteration_str_as_chars = iteration_string.chars().collect::<Vec<char>>();

    for i in 0..iteration_str_as_chars.len() {
        iteration += 1;
        
        let c = iteration_str_as_chars[i];

        previous_characters.push_back(c);

        if has_unique_elements(&previous_characters) {
            // println!("Uniq!: {:?}", previous_characters);
            println!("Iteration {}", iteration);
            return Ok(i + target_length);
        }

        // println!("Not uniq! {:?}", previous_characters);

        previous_characters.pop_front();
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
    #[ignore]
    fn part_1_has_right_answer() {
        assert_eq!(find_start_of_packet_marker_index().unwrap(), 1816);
    }


    #[test]
    #[ignore]
    fn part_2_has_right_answer() {
        assert_eq!(find_start_of_message_marker_index().unwrap(), 2625);
    }
}