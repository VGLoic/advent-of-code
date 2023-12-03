pub fn find_number_of_resting_units_of_sand(filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
    Ok(3)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1_should_give_expected_answer() {
        assert_eq!(
            find_number_of_resting_units_of_sand("inputs/input-14-example.txt").unwrap(),
            24
        );
    }
}
