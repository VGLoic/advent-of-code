use std::fs;

pub fn count_fully_contained_assignement_in_pair(filename: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filename)?;

    let mut count = 0;

    for line in contents.lines() {
        let pair = parse_line_into_pair(line)?;

        if pair.has_contained_assignements() {
            count += 1;
        }
    }

    return Ok(count);
}

pub fn count_overlapping_assignement_in_pair(filename: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filename)?;

    let mut count = 0;

    for line in contents.lines() {
        let pair = parse_line_into_pair(line)?;

        if pair.has_overlapping_assignemments() {
            count += 1;
        }
    }

    return Ok(count);
}

fn parse_line_into_pair(line: &str) -> Result<Pair, Box<dyn std::error::Error>> {
    let raw_assignements = line.trim().trim_end().split(",").collect::<Vec<&str>>();
    if raw_assignements.len() != 2 {
        return Err("Expected two assignements from line".into());
    }

    let pair = Pair {
        a: Assignement::from_raw(raw_assignements[0])?,
        b: Assignement::from_raw(raw_assignements[1])?,
    };
    return Ok(pair);
}

#[derive(Debug, Copy, Clone)]
struct Assignement {
    start: u32,
    end: u32,
}

impl Assignement {
    fn from_raw(raw: &str) -> Result<Assignement, Box<dyn std::error::Error>> {
        let boundaries: Vec<_> = raw
            .split("-")
            .map(|x| {
                return x.parse::<u32>();
            })
            .collect::<Result<Vec<_>, _>>()?;

        if boundaries.len() != 2 {
            return Err("Expected two buondaries in raw assignement".into());
        }

        return Ok(Assignement {
            start: boundaries[0],
            end: boundaries[1],
        });
    }
}

#[derive(Debug)]
struct Pair {
    a: Assignement,
    b: Assignement,
}

impl Pair {
    fn has_contained_assignements(&self) -> bool {
        let (first, second) = order_assignements(&self.a, &self.b);
        return second.end <= first.end;
    }

    fn has_overlapping_assignemments(&self) -> bool {
        let (first, second) = order_assignements(&self.a, &self.b);
        return first.end >= second.start;
    }
}

fn order_assignements<'a>(
    a: &'a Assignement,
    b: &'a Assignement,
) -> (&'a Assignement, &'a Assignement) {
    if a.start < b.start {
        return (a, b);
    }
    if a.start == b.start {
        if a.end < b.end {
            return (b, a);
        }
        return (a, b);
    }
    return (b, a);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1_has_right_answer() {
        assert_eq!(count_fully_contained_assignement_in_pair("inputs/input-04-example.txt").unwrap(), 2);
    }

    #[test]
    fn part_1_has_right_answer() {
        assert_eq!(count_fully_contained_assignement_in_pair("inputs/input-04.txt").unwrap(), 450);
    }

    #[test]
    fn example_part_2_has_right_answer() {
        assert_eq!(count_overlapping_assignement_in_pair("inputs/input-04-example.txt").unwrap(), 4);
    }

    #[test]
    fn part_2_has_right_answer() {
        assert_eq!(count_overlapping_assignement_in_pair("inputs/input-04.txt").unwrap(), 837);
    }
}
