use std::fs;

pub fn sum_signal_strengths(filename: &str) -> Result<isize, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filename)?;

    let mut cpu = CPU::new();

    let mut next_cycle_of_interest: usize = 20;

    let mut sum_signal_strength: isize = 0;

    let mut lines = contents.lines();

    loop {    
        if cpu.cycle == next_cycle_of_interest {
            sum_signal_strength += cpu.register * isize::try_from(next_cycle_of_interest).unwrap();
            next_cycle_of_interest += 40;
        }

        if !cpu.is_executing() {
            let next_line = lines.next();
            match next_line {
                None => break,
                Some(line) => {
                    let instruction = Instruction::try_from(line)?;
                    cpu.begin_execution(&instruction);
                    println!("Cycle {}\nRegister value: {}\nStart execution of {:?}\n", cpu.cycle, cpu.register, instruction);
                }
            }
        } else {
            println!("Cycle {}\nRegister value: {}\n", cpu.cycle, cpu.register);
        }

        cpu.tick();
    }

    Ok(sum_signal_strength)
}

pub fn display_signal(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filename)?;

    let mut cpu = CPU::new();

    let mut lines = contents.lines();

    let mut result = "".to_owned();

    loop {
        if !cpu.is_executing() {
            let next_line = lines.next();
            match next_line {
                None => break,
                Some(line) => {
                    let instruction = Instruction::try_from(line)?;
                    cpu.begin_execution(&instruction);
                    println!("Cycle {}\nRegister value: {}\nStart execution of {:?}\n", cpu.cycle, cpu.register, instruction);
                }
            }
        } else {
            println!("Cycle {}\nRegister value: {}\n", cpu.cycle, cpu.register);
        }

        let cursor_position = (isize::try_from(cpu.cycle).unwrap() - 1) % 40;

        println!("Cursor position: {}\nLower boundary {}\nHigher boundary {} \n", cursor_position, cpu.register - 1, cpu.register + 1);
        if cursor_position >= cpu.register - 1 && cursor_position <= cpu.register + 1 {
            result += "#";
        } else {
            result += ".";
        }

        if cpu.cycle % 40 == 0 {
            result += "\n";
        }

        cpu.tick();
    }

    Ok(result)
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(isize)
}

impl TryFrom<&str> for Instruction {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "noop" => Ok(Instruction::Noop),
            other => {
                let trimmed_line = other.trim().trim_end();
                let add_prefix = "addx ";
                if !trimmed_line.starts_with(add_prefix) {
                    return Err(format!("Unable to parse string into instruction, got {}, expected `noop` or `addx <value>`", other).into());
                }

                let v= trimmed_line
                    .strip_prefix(add_prefix)
                    .and_then(|x| {
                        let parsed_result = x.parse::<isize>();
                        match parsed_result {
                            Ok(a) => Some(a),
                            Err(_) => None
                        }
                    });
                    
                match v {
                    None => Err(format!("Unable to parse string into instruction, expected a numeric value after the `addx ` part").into()),
                    Some(x) =>  Ok(Instruction::Addx(x))
                }
            }
        }
    }
}

#[derive(Debug)]
struct NextState {
    cycle: usize,
    register: isize
}

#[derive(Debug)]
struct CPU {
    cycle: usize,
    register: isize,
    executing: Option<NextState>,
}

impl CPU {
    fn is_executing(&self) -> bool {
        return self.executing.is_some();
    }

    fn tick(&mut self) {
        self.cycle += 1;
        if let Some(next_state) = &self.executing {
            if next_state.cycle == self.cycle {
                self.register = next_state.register;
                self.executing = None;
            }
        }
    }

    fn begin_execution(&mut self, instruction: &Instruction) {
        let next_state = match instruction {
            Instruction::Noop => NextState { cycle: self.cycle + 1, register: self.register },
            Instruction::Addx(v) => {
                NextState { cycle: self.cycle + 2, register: self.register + v }
            }
        };
        self.executing = Some(next_state);
    }
}

impl CPU {
    fn new() -> Self {
        CPU { register: 1, cycle: 1, executing: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1_should_give_expected_result() {
        assert_eq!(
            sum_signal_strengths("inputs/input-10-example.txt").unwrap(),
            13140
        );
    }

    #[test]
    fn part_1_should_give_expected_result() {
    assert_eq!(
        sum_signal_strengths("inputs/input-10.txt").unwrap(),
        14860
    );
    }

    #[test]
    fn example_part_2_should_give_expected_result() {
        assert_eq!(
            display_signal("inputs/input-10-example.txt").unwrap(),
"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....\n".to_owned()
        )
    }

    #[test]
    fn part_2_should_give_expected_result() {
        assert_eq!(
            display_signal("inputs/input-10.txt").unwrap(),
"###...##..####.####.#..#.#..#.###..#..#.
#..#.#..#....#.#....#..#.#..#.#..#.#.#..
#..#.#......#..###..####.#..#.#..#.##...
###..#.##..#...#....#..#.#..#.###..#.#..
#.#..#..#.#....#....#..#.#..#.#.#..#.#..
#..#..###.####.####.#..#..##..#..#.#..#.\n".to_owned()
        )
    }
}
