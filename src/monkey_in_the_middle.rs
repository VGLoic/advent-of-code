use std::{fs, collections::{VecDeque, HashMap}};

pub fn compute_monkey_business(filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(filename)?;

    let mut monkeys: Vec<Monkey> = vec![];
    for monkey_str in content.split("\n\n") {
        monkeys.push(Monkey::try_from(monkey_str)?);
    }

    let number_of_monkeys = monkeys.len();

    for i in 0..20 {
        println!("Round {i}");
        for i in 0..number_of_monkeys {
            println!("  Monkey {i}");
            let monkey = &mut monkeys[i];
            println!("      Has {} items", monkey.items.len());
            let mut thrown_items = vec![];
            while monkey.has_items() {
                let (thrown_item, destination_monkey_index) = monkey.inspect_next_item(3)?;
                thrown_items.push(
                    (thrown_item, destination_monkey_index)
                );
            }
            for (complexity, destination_index) in thrown_items {
                monkeys[destination_index].receive_new_item(complexity);
            }
        }
    }

    let mut counts = monkeys.iter().map(|m| m.inspected_items_count).collect::<Vec<usize>>();
    counts.sort_unstable();
    counts.reverse();
    
    Ok(counts[0] * counts[1])
}

pub fn compute_big_monkey_business(filename: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(filename)?;

    let monkey_reunion = &mut BigMonkeysReunion::try_from(content.as_str())?;

    for i in 0..10_000 {
        println!("Round {i}");
        monkey_reunion.play_round();
    }

    let mut counts = monkey_reunion.monkeys.iter().map(|m| m.inspected_items_count).collect::<Vec<usize>>();
    counts.sort_unstable();
    counts.reverse();
    
    Ok(counts[0] * counts[1])
}

#[derive(Debug)]
struct BigMonkeysReunion {
    item_test_cache: HashMap<(usize, usize), (usize, usize)>,
    items: HashMap<usize, BigMonkeyItem>,
    monkeys: Vec<BigMonkey>,
}

impl BigMonkeysReunion {
    fn play_round(&mut self) {
        for i in 0..self.monkeys.len() {
            println!("  Monkey {i}");
            
            let mut thrown_items = vec![];
            
            let monkey = &mut self.monkeys[i];
            println!("      Has {} items", monkey.items.len());

            for item_id in &monkey.items {
                let item = self.items.get_mut(&item_id).unwrap();
                item.operations.push(monkey.operation.clone());

                let remainder = item.remainder(monkey.test.divider, self.item_test_cache.get(&(*item_id, i)));
                self.item_test_cache.insert((*item_id, i), (remainder, item.operations.len()));

                let destination_index = if remainder == 0 {
                    monkey.test.test_true_destination_index
                } else {
                    monkey.test.test_false_destination_index
                };
                
                thrown_items.push((*item_id, destination_index));
            }

            monkey.inspected_items_count += monkey.items.len();
            monkey.items.clear();


            for (item_id, destination_index) in thrown_items {
                self.monkeys[destination_index].items.push(item_id);
            }
        }
    }
}

impl TryFrom<&str> for BigMonkeysReunion {
    type Error = Box<dyn std::error::Error>;
  
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut monkeys: Vec<BigMonkey> = vec![];
        let mut items = HashMap::<usize, BigMonkeyItem>::new();

        let mut item_index = 0;

        for monkey_str in value.split("\n\n") {
            let mut lines = monkey_str.lines();
    
            // Skip first line
            lines.next();
    
            // Second line is for the matching items
            let second_line = lines.next().ok_or("Line for matching items definition not found")?.trim().trim_end();
            let items_prefix = "Starting items:";
            if !second_line.starts_with(items_prefix) {
            return Err(format!("Invalid line, expected {} <items>, got `{}`", items_prefix, second_line).into());
            }
            let monkey_initial_items = second_line
                .strip_prefix(items_prefix).or(Some("")).unwrap()
                .split(",")
                .map(|x| x.trim().parse::<usize>())
                .map(|x| x.and_then(|n| Ok(BigMonkeyItem{
                    initial_worry_level: n,
                    operations: vec![]
                })))
                .collect::<Result<Vec<BigMonkeyItem>, std::num::ParseIntError>>()
                .map_err(|_| "Unable to parse one of the given item into a number")?;

            let mut monkey_initial_items_ids = vec![];
            for item in monkey_initial_items {
                items.insert(item_index, item);
                monkey_initial_items_ids.push(item_index);
                item_index += 1;
            }

    
            // Third line is for the operation
            let third_line = lines.next().ok_or("Line for operation definition not found")?.trim().trim_end();
            let operation = Operation::try_from(third_line)?;
            
            // Remaining lines are for the test
            let monkey_test_lines = lines.collect::<Vec<&str>>().join("\n");
            let monkey_test = MonkeyTest::try_from(
                monkey_test_lines.as_str()
            )?;

            monkeys.push(BigMonkey {
                inspected_items_count: 0,
                items: monkey_initial_items_ids,
                operation,
                test: monkey_test
            });
        }

        Ok(BigMonkeysReunion { item_test_cache: HashMap::new(), items, monkeys })
    }
}

#[derive(Debug)]
struct BigMonkey {
    inspected_items_count: usize,
    items: Vec<usize>,
    operation: Operation,
    test: MonkeyTest,
}

#[derive(Debug, Clone)]
struct BigMonkeyItem {
    initial_worry_level: usize,
    operations: Vec<Operation>
}

// Thread implementation
impl BigMonkeyItem {
    fn remainder(&self, divider: usize, cache: Option<&(usize, usize)>) -> usize {
        let mut remainder = cache.and_then(|c| Some(c.0)).unwrap_or(self.initial_worry_level % divider);
        let start_index = cache.and_then(|c| Some(c.1)).unwrap_or(0);
        for i in start_index..self.operations.len() {
            let op = &self.operations[i];
            match op {
                Operation::Addition(v) => {
                    match v {
                        OperationValue::Itself => {
                            remainder = (remainder * 2) % divider;
                        },
                        OperationValue::Value(n) => {
                            remainder = (remainder + n) % divider;
                        }
                    }
                },
                Operation::Multiplication(v) => {
                    match v {
                        OperationValue::Itself => {
                            remainder = (remainder * remainder) % divider;
                        },
                        OperationValue::Value(n) => {
                            remainder = (remainder * n) % divider;
                        }
                    }
                }
            }
        }
        remainder
    }
}

#[derive(Debug)]
struct Monkey {
    inspected_items_count: usize,
    items: VecDeque<usize>,
    operation: Operation,
    test: MonkeyTest,
}

impl Monkey {
    fn has_items(&self) -> bool {
        return self.items.len() > 0;
    }

    fn inspect_next_item(&mut self, worry_divider: usize) -> Result<(usize, usize), Box<dyn std::error::Error>> {
        let item = self.items.pop_front().ok_or("No next item found")?;

        self.inspected_items_count += 1;

        println!("  Monkey inspects an item with a worry level of {item}.");

        let mut new_worry_level = self.compute_new_worry_level(item);
        println!("      New worry level is {new_worry_level}.");

        new_worry_level = new_worry_level / worry_divider;
        println!("      Monkey gets bored with item. Worry level is divided by {worry_divider} to {new_worry_level}.");

        let pass_test = &new_worry_level % self.test.divider == 0_usize;

        if pass_test {
            println!("      Current worry level is divisible by {}.", self.test.divider);
            println!("      Item with worry level {} is thrown to monkey {}.", new_worry_level, self.test.test_true_destination_index);
            return Ok((new_worry_level, self.test.test_true_destination_index));
        } else {
            println!("      Current worry level is divisible by {}.", self.test.divider);
            println!("      Item with worry level {} is thrown to monkey {}.", new_worry_level, self.test.test_false_destination_index);
            return Ok((new_worry_level, self.test.test_false_destination_index));
        }
    }

    fn receive_new_item(&mut self, item: usize) {
        self.items.push_back(item);
    }

    fn compute_new_worry_level(&self, item: usize) -> usize {
        match &self.operation {
            Operation::Addition(v) => {
                match v {
                    OperationValue::Itself => item * 2_usize,
                    OperationValue::Value(n) => item + n
                }
            },
            Operation::Multiplication(v) => {
                match v {
                    OperationValue::Itself => item * item,
                    OperationValue::Value(n) => item * n
                }
            }
        }
    }
}

impl TryFrom<&str> for Monkey {
    type Error = Box<dyn std::error::Error>;
  
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines = value.lines();
  
        // Skip first line
        lines.next();
  
        // Second line is for the matching items
        let second_line = lines.next().ok_or("Line for matching items definition not found")?.trim().trim_end();
        let items_prefix = "Starting items:";
        if !second_line.starts_with(items_prefix) {
          return Err(format!("Invalid line, expected {} <items>, got `{}`", items_prefix, second_line).into());
        }
        let items = second_line
          .strip_prefix(items_prefix).or(Some("")).unwrap()
          .split(",")
          .map(|x| x.trim().parse::<usize>())
          .collect::<Result<Vec<usize>, std::num::ParseIntError>>()
          .map_err(|_| "Unable to parse one of the given item into a number")?;

  
        // Third line is for the operation
        let third_line = lines.next().ok_or("Line for operation definition not found")?.trim().trim_end();
        let operation = Operation::try_from(third_line)?;
        
        // Remaining lines are for the test
        let monkey_test_lines = lines.collect::<Vec<&str>>().join("\n");
        let monkey_test = MonkeyTest::try_from(
          monkey_test_lines.as_str()
        )?;
  
        Ok(Monkey { inspected_items_count: 0, items: VecDeque::from(items), operation, test: monkey_test })
    }
}
  

#[derive(Debug)]
struct MonkeyTest {
    divider: usize,
    test_true_destination_index: usize,
    test_false_destination_index: usize
}

impl TryFrom<&str> for MonkeyTest {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines = value.lines();

        // First line is for the divider of the test
        let first_line = lines.next().ok_or("Line for test divider definition not found")?.trim().trim_end();
        let divider_prefix = "Test: divisible by";
        if !first_line.starts_with(divider_prefix) {
            return Err(format!("Invalid line, expected {} <divider>, got `{}`", divider_prefix, first_line).into());
        }
        let divider = first_line
            .strip_prefix(divider_prefix)
            .unwrap_or("")
            .trim().trim_end()
            .parse::<usize>()
            .map_err(|_| "Unable to parse divider value, expected a number")?;

        // Second line is for the destination if test succeeds
        let second_line = lines.next().ok_or("Line for test success destination definition not found")?.trim().trim_end();
        let true_destination_prefix = "If true: throw to monkey";
        if !second_line.starts_with(true_destination_prefix) {
            return Err(format!("Invalid line, expected {} <monkey index>, got `{}`", true_destination_prefix, second_line).into());
        }
        let test_true_destination_index = second_line
            .strip_prefix(true_destination_prefix)
            .unwrap_or("")
            .trim().trim_end()
            .parse::<usize>()
            .map_err(|_| "Unable to parse monkey test true destination index value, expected a number")?;

        // Third line is for the destination if test fails
        let third_line = lines.next().ok_or("Line for test failure destination definition not found")?.trim().trim_end();
        let false_destination_prefix = "If false: throw to monkey";
        if !third_line.starts_with(false_destination_prefix) {
            return Err(format!("Invalid line, expected {} <monkey index>, got `{}`", false_destination_prefix, third_line).into());
        }
        let test_false_destination_index = third_line
            .strip_prefix(false_destination_prefix)
            .unwrap_or("")
            .trim().trim_end()
            .parse::<usize>()
            .map_err(|_| "Unable to parse monkey test false destination index value, expected a number")?;

        Ok(MonkeyTest{ divider, test_false_destination_index, test_true_destination_index })
    }
}

#[derive(Debug, Clone)]
enum OperationValue {
    Itself,
    Value(usize)
}

#[derive(Debug, Clone)]
enum Operation {
    Addition(OperationValue),
    Multiplication(OperationValue),
}

impl TryFrom<&str> for Operation {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let operation_prefix = "Operation: new = old";
        if !value.starts_with(operation_prefix) {
            return Err(format!("2 Invalid value for Operation, expected `{} <+ x | * x>`, got `{}`", operation_prefix, value).into());
        }

        let elements = value.strip_prefix(operation_prefix).unwrap_or("").trim().trim_end().split(" ").collect::<Vec<&str>>();
        if elements.len() != 2 {
            return Err(format!("Invalid value for Operation, expected `{} <+ x | * x>`, got `{}`", operation_prefix, value).into());
        }

        let operation_value = match elements[1] {
            "old" => OperationValue::Itself,
            other => {
                let value = other.parse::<usize>()
                    .map_err(|_| format!("Unable to parse operation value, expected a number, got {}", other))?;
                OperationValue::Value(value)
            }
        };

        match elements[0] {
            "+" => Ok(Operation::Addition(operation_value)),
            "*" => Ok(Operation::Multiplication(operation_value)),
            other => Err(format!("Unsupported symbol for operation, expected `+` or `*`, got {}", other).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1_should_give_expected_result() {
        assert_eq!(
            compute_monkey_business("inputs/input-11-example.txt").unwrap(),
            10605
        );
    }

    #[test]
    fn part_1_should_give_expected_result() {
        assert_eq!(
            compute_monkey_business("inputs/input-11.txt").unwrap(),
            110264
        );
    }

    #[test]
    fn example_part_2_should_give_expected_result() {
        assert_eq!(
            compute_big_monkey_business("inputs/input-11-example.txt").unwrap(),
            2713310158
        );
    }

    #[test]
    fn part_2_should_give_expected_result() {
        assert_eq!(
            compute_big_monkey_business("inputs/input-11.txt").unwrap(),
            23612457316
        );
    }
}
