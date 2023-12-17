use std;

pub fn find_most_released_pressure(filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
    Ok(3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1_should_give_expected_result() {
        assert_eq!(
            find_most_released_pressure("inputs/input-16-example.txt").unwrap(),
            1651
        );
    }
}
