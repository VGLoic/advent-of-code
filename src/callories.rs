use std::fs;


pub fn find_elf_with_most_callories_v2() -> u32 {
    let contents = fs::read_to_string("input-01.txt").expect("Expected a file `input-01.txt`");
    let mut max = 0;
    let mut elf_sum = 0;
    contents.lines().for_each(|line| {
        if line.len() > 0 {
            let callory: u32 = line.parse().expect("Unable to parse line as u32");
            elf_sum += callory;
            return;
        }

        if elf_sum > max {
            max = elf_sum;
        }
        elf_sum = 0;
    });

    return max;
}

pub fn find_sum_of_three_most_callories_v2() -> u32 {
    let contents = fs::read_to_string("input-01.txt").expect("Expected a file `input-01.txt`");
    let mut maximums = vec![0, 0, 0];
    let mut elf_sum = 0;
    contents.lines().for_each(|line| {
        if line.len() > 0 {
            let callory: u32 = line.parse().expect("Unable to parse line as u32");
            elf_sum += callory;
            return;
        }

        if elf_sum > maximums[0] {
            maximums[0] = elf_sum;
            maximums.sort_unstable();
        }

        elf_sum = 0;
    });

    return maximums.iter().sum();
}
