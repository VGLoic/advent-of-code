use std::{
    fmt::{self},
    fs, str, vec,
};

pub fn move_crates(should_move_crate_one_at_the_time: bool) -> Result<String, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("input-05.txt")?;

    let index = contents
        .find("\n\n")
        .ok_or("Unable to find double line break")?;

    let (crates_config, orders_config) = contents.split_at(index);

    println!("crates_confi: \n{}\n", crates_config);

    let mut crates_setup = parse_crates_setup(crates_config)?;

    println!("crates setup: {}", crates_setup);

    let orders = parse_orders(orders_config)?;

    for order in orders {
        if should_move_crate_one_at_the_time {
            crates_setup.apply_order_one_crate_at_the_time(&order)?;
        } else {
            crates_setup.apply_order_multiple_crates_at_the_time(&order)?;
        }

        println!("Order: {}", order);
        println!("crates setup: {}", crates_setup);
    }

    let last_elements: Result<Vec<_>, _> = crates_setup
        .setup
        .iter()
        .map(|c| c.last().ok_or("Oh no"))
        .collect();

    let mut concat = "".to_string();
    for el in last_elements? {
        let unwrapped_element = el.get(1..2).ok_or("Unable to extract element in bracket")?;
        concat.push_str(unwrapped_element);
    }

    return Ok(concat);
}

struct CratesSetup {
    setup: Vec<Vec<String>>,
}

impl CratesSetup {
    fn apply_order_one_crate_at_the_time(&mut self, order: &Order) -> Result<(), Box<dyn std::error::Error>> {
        for _ in 0..order.quantity {
            let moved_crate = self.setup[order.from]
                .last()
                .ok_or("No last element found")?
                .to_string();
            self.setup[order.to].push(moved_crate);
            self.setup[order.from].pop();
        }
        return Ok(());
    }

    fn apply_order_multiple_crates_at_the_time(&mut self, order: &Order) -> Result<(), Box<dyn std::error::Error>> {
        let from_column_length = self.setup[order.from].len();
        let mut moved_crates: Vec<_> = self.setup[order.from]
            .drain(from_column_length - order.quantity..)
            .collect();
        if moved_crates.len() != order.quantity {
            return Err(format!(
                "Unexpected number of moved crates, expected {}, got {}",
                order.quantity,
                moved_crates.len()
            )
            .into());
        }

        self.setup[order.to].append(&mut moved_crates);
        return Ok(());
    }
}

impl fmt::Display for CratesSetup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut displayed_string_vec: Vec<String> = vec![];
        let mut first_line = "".to_string();
        for i in 0..self.setup.len() {
            first_line.push_str(&format!(" {} ", i + 1));
            first_line.push_str(" ");
        }

        displayed_string_vec.push(first_line);

        let highest_vec = self.setup.iter().map(|c| c.len()).max();

        match highest_vec {
            Some(highest_vec_size) => {
                for i in 0..highest_vec_size {
                    let mut displayed_line = "".to_string();
                    for j in 0..self.setup.len() {
                        if i >= self.setup[j].len() {
                            displayed_line.push_str("   ");
                        } else {
                            displayed_line.push_str(&self.setup[j][i]);
                        }
                        displayed_line.push_str(" ");
                    }
                    displayed_string_vec.push(displayed_line);
                }

                displayed_string_vec.reverse();

                return write!(f, "\n{}\n", displayed_string_vec.join("\n"));
            }
            None => {
                return write!(f, "An issue occurred while displaying the Crates Setup!\n");
            }
        }
    }
}

fn parse_crates_setup(crates_config: &str) -> Result<CratesSetup, Box<dyn std::error::Error>> {
    let mut crates_setup: Vec<Vec<String>> = vec![];

    let mut is_first_line = true;

    let mut number_of_column: usize = 0;

    for line in crates_config.lines().rev() {
        if is_first_line {
            number_of_column = line
                .trim()
                .trim_end()
                .split_whitespace()
                .last()
                .ok_or("Unable to find last element of column configuration line")?
                .parse::<usize>()?;
            let mut i = 0;
            while i < number_of_column {
                crates_setup.push(vec![]);
                i += 1;
            }
            is_first_line = false;
        } else {
            let mut i: usize = 0;
            while i < number_of_column {
                let part = line.chars().skip(4 * i).take(3).collect::<String>();
                let trimed_part = part.trim().trim_end();
                if !trimed_part.is_empty() {
                    crates_setup[i].push(trimed_part.to_string());
                }
                i += 1;
            }
        }
    }

    return Ok(CratesSetup {
        setup: crates_setup,
    });
}

#[derive(Debug)]
struct Order {
    quantity: usize,
    from: usize,
    to: usize,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "Move {} from {} to {}\n",
            self.quantity,
            self.from + 1,
            self.to + 1
        );
    }
}

fn parse_orders(orders_config: &str) -> Result<Vec<Order>, Box<dyn std::error::Error>> {
    let mut orders: Vec<Order> = vec![];

    for line in orders_config.lines() {
        let trimed_line = line.trim().trim_end();
        if trimed_line.is_empty() {
            continue;
        }
        let elements: Vec<_> = trimed_line.split(" ").collect();
        if elements.len() != 6 {
            return Err(format!("Expected 6 elements in line, got {}", elements.len()).into());
        }
        let quantity = elements[1].parse::<usize>()?;
        let from = elements[3].parse::<usize>()?;
        let to = elements[5].parse::<usize>()?;

        orders.push(Order {
            quantity: quantity,
            from: from - 1,
            to: to - 1,
        });
    }

    return Ok(orders);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_has_right_answer() {
        assert_eq!(move_crates(true).unwrap(), "SHMSDGZVC");
    }

    #[test]
    fn part_2_has_right_answer() {
        assert_eq!(move_crates(false).unwrap(), "VRZGHDFBQ");
    }
}