use std;

pub fn sum_over_right_pair_indices(filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;

    let mut sum = 0;
    let mut pair_index = 1;

    for raw_pair in content.split("\n\n") {
        let mut packets = vec![];
        for side_line in raw_pair.lines() {
            if packets.len() > 2 {
                return Err(format!(
                    "Invalid pair input, expected at most two lines, got at least three"
                )
                .into());
            }
            if !side_line.starts_with("[") || !side_line.ends_with("]") {
                return Err(format!(
                    "Invalid side string, expected to have format `[...]`, got {}",
                    side_line
                )
                .into());
            }

            println!("Line {side_line}");
            let packet = Packet::try_from(side_line)?;
            // println!("Got items {:?}\n\n", side.items);

            packets.push(packet);

            if packets.len() == 2 {
                let left_side = &packets[0];
                let right_side = &packets[1];

                let right_order = left_side.are_items_in_right_order(right_side);

                if right_order {
                    sum += pair_index;
                }

                println!(
                    "Got both sides:
    Index {pair_index}
    Order good: {right_order}
                "
                );

                pair_index += 1;
            }
        }
    }

    Ok(sum)
}

struct Packet {
    items: Vec<Item>,
}

impl Packet {
    fn are_items_in_right_order(&self, right_packet: &Packet) -> bool {
        let mut index = 0;
        for item in &self.items {
            if index >= right_packet.items.len() {
                return false;
            }

            match item.compare(&right_packet.items[index]) {
                ComparisonResult::RightOrder => {
                    return true;
                }
                ComparisonResult::WrongOrder => {
                    return false;
                }
                ComparisonResult::Undecided => {}
            };

            index += 1;
        }

        return true;
    }
}

impl TryFrom<&str> for Packet {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.starts_with("[") || !value.ends_with("]") {
            return Err(format!(
                "Invalid packet string, expected to have format `[...]`, got {}",
                value
            )
            .into());
        }

        let stripped_value = value.strip_prefix("[").and_then(|s| s.strip_suffix("]"));

        match stripped_value {
            None => Ok(Packet { items: vec![] }),
            Some(v) => {
                let items = parse_items(v)?;
                return Ok(Packet { items });
            }
        }
    }
}

fn parse_items(value: &str) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    let mut items: Vec<Item> = vec![];

    let mut item_start_index = 0;
    let mut index = 0;
    let mut depth = 0;
    for c in value.chars() {
        match c {
            '[' => {
                depth += 1;
                // println!("Increasing depth to {depth}");
            }
            ']' => {
                depth -= 1;
                // println!("Decreasing depth to {depth}");
            }
            ',' => {
                if depth == 0 {
                    // println!("Got new item! {}", &v[item_start_index..index]);
                    items.push(Item::try_from(&value[item_start_index..index])?);
                    item_start_index = index + 1;
                }
            }
            _ => {
                // println!("Iterating only... Value is {other}");
            }
        };
        index += 1;
    }
    if index > item_start_index {
        items.push(Item::try_from(&value[item_start_index..index])?);
    }

    Ok(items)
}

#[derive(Debug)]
enum Item {
    Value(usize),
    List(Vec<Item>),
}

enum ComparisonResult {
    RightOrder,
    WrongOrder,
    Undecided,
}

impl Item {
    fn compare(&self, right_item: &Item) -> ComparisonResult {
        // println!("Compare {:?} with {:?}", self, right_item);
        match self {
            Item::Value(a) => {
                match right_item {
                    Item::Value(b) => {
                        if a < b {
                            // println!("Got success when comparing {a} with {b}");
                            return ComparisonResult::RightOrder;
                        }
                        if a > b {
                            // println!("Got failure when comparing {a} with {b}");
                            return ComparisonResult::WrongOrder;
                        }
                        return ComparisonResult::Undecided;
                    }
                    Item::List(_) => {
                        let upgraded_left_item = Item::List(vec![Item::Value(*a)]);
                        return upgraded_left_item.compare(right_item);
                    }
                }
            }
            Item::List(left_l) => {
                match right_item {
                    Item::Value(b) => {
                        let upgraded_right_item = Item::List(vec![Item::Value(*b)]);
                        return self.compare(&upgraded_right_item);
                    }
                    Item::List(right_l) => {
                        let right_item_len = right_l.len();

                        let mut i = 0;
                        for item in left_l {
                            if i >= right_item_len {
                                // println!("Right is too short {:?} with {:?}", left_l, right_item);
                                return ComparisonResult::WrongOrder;
                            }

                            match item.compare(&right_l[i]) {
                                ComparisonResult::RightOrder => {
                                    // println!("Right order for {:?} with {:?}", left_l, right_item);
                                    return ComparisonResult::RightOrder;
                                }
                                ComparisonResult::WrongOrder => {
                                    // println!("Wrong order for {:?} with {:?}", left_l, right_item);
                                    return ComparisonResult::WrongOrder;
                                }
                                ComparisonResult::Undecided => {}
                            };

                            i += 1;
                        }

                        if i == right_item_len {
                            return ComparisonResult::Undecided;
                        }

                        // println!("Left side ran out of items so good {:?} & {:?}", left_l, right_l);
                        return ComparisonResult::RightOrder;
                    }
                }
            }
        }
    }
}

impl TryFrom<&str> for Item {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // println!("Building item from {value}");

        let is_nested = value.starts_with("[") && value.ends_with("]");

        if !is_nested {
            let v = value.parse::<usize>()?;
            return Ok(Item::Value(v));
        }

        // Else we open the value
        let stripped_value = value
            .strip_prefix("[")
            .and_then(|v| v.strip_suffix("]"))
            .unwrap();
        let items = parse_items(stripped_value)?;
        Ok(Item::List(items))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1_has_right_answer() {
        assert_eq!(
            sum_over_right_pair_indices("inputs/input-13-example.txt").unwrap(),
            13
        );
    }

    #[test]
    fn part_1_has_right_answer() {
        assert_eq!(
            sum_over_right_pair_indices("inputs/input-13.txt").unwrap(),
            5682
        );
    }
}
