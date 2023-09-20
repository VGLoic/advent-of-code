use std::fs;

pub fn compute_score_with_initial_strategy() -> Result<u32, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("input-02.txt")?;
    let mut score = 0;
    for line in contents.lines() {
        score += Round::build_using_first_strategy(line)?.score();
    }
    Ok(score)
}

pub fn compute_score_with_second_strategy() -> Result<u32, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string("input-02.txt")?;
    let mut score = 0;
    for line in contents.lines() {
        score += Round::build_using_second_strategy(line)?.score();
    }
    Ok(score)
}

#[derive(Debug)]
enum GameChoice {
    Rock,
    Paper,
    Scissors,
}

impl GameChoice {
    fn to_num(&self) -> u32 {
        match self {
            GameChoice::Rock => 0,
            GameChoice::Paper => 1,
            GameChoice::Scissors => 2,
        }
    }
    fn from_num(n: u32) -> Result<GameChoice, &'static str> {
        match n {
            0 => Ok(GameChoice::Rock),
            1 => Ok(GameChoice::Paper),
            2 => Ok(GameChoice::Scissors),
            _ => Err("Invalid num entry, expected a number from 0 to 2"),
        }
    }

    fn build_other_player_choice(value: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match value {
            "A" => Ok(GameChoice::Rock),
            "B" => Ok(GameChoice::Paper),
            "C" => Ok(GameChoice::Scissors),
            other => Err(format!("Invalid choice, expected A, B or C, got {}", other).into()),
        }
    }

    fn build_using_first_strategy(value: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match value {
            "X" => Ok(GameChoice::Rock),
            "Y" => Ok(GameChoice::Paper),
            "Z" => Ok(GameChoice::Scissors),
            other => Err(format!("Invalid choice, expected X, Y or Z, got {}", other).into()),
        }
    }

    fn build_using_second_strategy(
        value: &str,
        other_player_choice: &GameChoice,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let result_needed = ResultNeed::try_from(value)?;
        let other_player_choice_num = other_player_choice.to_num();
        match result_needed {
            ResultNeed::Loose => Ok(GameChoice::from_num((other_player_choice_num + 2) % 3)?),
            ResultNeed::Draw => Ok(GameChoice::from_num(other_player_choice_num)?),
            ResultNeed::Win => Ok(GameChoice::from_num((3 + other_player_choice_num + 1) % 3)?),
        }
    }
}

#[derive(Debug)]
struct Round {
    what_the_other_played: GameChoice,
    what_i_played: GameChoice,
}

impl Round {
    fn new(what_i_played: GameChoice, what_the_other_played: GameChoice) -> Round {
        Round {
            what_i_played,
            what_the_other_played,
        }
    }

    fn build_using_first_strategy(value: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let choices: Vec<_> = value.trim().trim_end().split(" ").collect();

        if choices.len() != 2 {
            return Err(format!(
                "Invalid line format, expect '<Letter> <Letter>', got {}",
                value
            )
            .into());
        }

        let what_the_other_played = GameChoice::build_other_player_choice(choices[0])?;
        let what_i_played = GameChoice::build_using_first_strategy(choices[1])?;

        let round = Round::new(what_i_played, what_the_other_played);

        return Ok(round);
    }

    fn build_using_second_strategy(value: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let choices: Vec<_> = value.trim().trim_end().split(" ").collect();

        if choices.len() != 2 {
            return Err(format!(
                "Invalid value format, expect '<Letter> <Letter>', got {}",
                value
            )
            .into());
        }

        let what_the_other_played = GameChoice::build_other_player_choice(choices[0])?;
        let what_i_played =
            GameChoice::build_using_second_strategy(choices[1], &what_the_other_played)?;

        let round = Round::new(what_i_played, what_the_other_played);

        return Ok(round);
    }

    fn base_score(&self) -> u32 {
        match self.what_i_played {
            GameChoice::Rock => 1,
            GameChoice::Paper => 2,
            GameChoice::Scissors => 3,
        }
    }
    fn compete_score(&self) -> u32 {
        let diff = (3 + self.what_the_other_played.to_num() - self.what_i_played.to_num()) % 3;

        match diff {
            0 => 3,
            1 => 0,
            2 => 6,
            _ => panic!("Unexpected diff value :( {}", diff),
        }
    }

    fn score(&self) -> u32 {
        return self.base_score() + self.compete_score();
    }
}

enum ResultNeed {
    Loose,
    Draw,
    Win,
}

impl TryFrom<&str> for ResultNeed {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(ResultNeed::Loose),
            "Y" => Ok(ResultNeed::Draw),
            "Z" => Ok(ResultNeed::Win),
            other => Err(format!("Invalid choice, expected X, Y or Z, got {}", other).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_strategy_gives_expected_score() {
        assert_eq!(compute_score_with_initial_strategy().unwrap(), 13565);
    }

    #[test]
    fn part_2_strategy_gives_expected_score() {
        assert_eq!(compute_score_with_second_strategy().unwrap(), 12424);
    }
}
