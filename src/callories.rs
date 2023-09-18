use std::fs;

pub fn find_max_callories_on_single_elf() -> Result<usize, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("input-01.txt")?;
    let mut max = 0;
    let mut elf_sum = 0;
    for line in contents.lines() {
        let is_new_elf = line.is_empty();
        if is_new_elf {
            if elf_sum > max {
                max = elf_sum;
            }
            elf_sum = 0;
        } else {
            let callory: usize = line.parse()?;
            elf_sum += callory;
        }
    }

    Ok(max)
}

pub fn find_sum_of_maximums_callories(number_of_elves_to_consider: usize) -> Result<usize, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("input-01.txt")?;
    let mut maximums = vec![0; number_of_elves_to_consider];
    let mut elf_sum = 0;
    for line in contents.lines() {
        let is_new_elf = line.is_empty();
        if is_new_elf {
            if elf_sum > maximums[0] {
                maximums[0] = elf_sum;
                maximums.sort_unstable();
            }
    
            elf_sum = 0;
        } else {
            let callory: usize = line.parse()?;
            elf_sum += callory;
        }
    }

    Ok(maximums.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_give_expected_maximum_callories_on_single_elf() {
        assert_eq!(find_max_callories_on_single_elf().unwrap(), 71471);
    }

    #[test]
    fn part_2_should_give_expected_sum_of_three_most_callories() {
        assert_eq!(find_sum_of_maximums_callories(3).unwrap(), 211189);
    }
}
