
#[allow(dead_code)]
pub mod first_part {
    use std::fs;

    use super::game;

    pub fn compute_score_with_initial_strategy() -> u32 {
        let contents = fs::read_to_string("input-02.txt").expect("Expected a file `input-02.txt`");
        let mut score = 0;
        contents.lines().for_each(|line| {
            let round_score = parse_line_to_round(line)
                .expect("Unable to parse the line for a round")
                .score();
            score += round_score;
        });
        return score;
    }

    fn parse_line_to_round(line: &str) -> Result<game::Round, &str> {
        let choices: Vec<_> = line.trim().trim_end().split(" ").collect();
    
        if choices.len() != 2 {
            return Err("Invalid line format, expect '<Letter> <Letter>'")
        }
    
        let what_the_other_played = game::parse_other_player_choice(choices[0])?;
        let what_i_played = parse_my_choice(choices[1])?;
    
        let round = game::Round::new(
            what_i_played,
            what_the_other_played,
        );
    
        return Ok(round);
    }

    fn parse_my_choice(c: &str) -> Result<game::GameChoice, &'static str> {
        match c {
            "X" => Ok(game::GameChoice::Rock),
            "Y" => Ok(game::GameChoice::Paper),
            "Z" => Ok(game::GameChoice::Scissors),
            _ => Err("Invalid choice, expected X, Y or Z"),
        }
    }
}

pub mod second_part {
    use std::fs;

    use super::game;
    
    pub fn compute_score_with_second_strategy() -> u32 {
        let contents = fs::read_to_string("input-02.txt").expect("Expected a file `input-02.txt`");
        let mut score = 0;
        contents.lines().for_each(|line| {
            let round_score = parse_line_to_round(line)
                .expect("Unable to parse the line for a round")
                .score();
            score += round_score;
        });
        return score;
    }

    fn parse_line_to_round(line: &str) -> Result<game::Round, &str> {
        let choices: Vec<_> = line.trim().trim_end().split(" ").collect();
    
        if choices.len() != 2 {
            return Err("Invalid line format, expect '<Letter> <Letter>'")
        }
    
        let what_the_other_played = game::parse_other_player_choice(choices[0])?;
        let what_i_played = parse_expected_result(choices[1])?.to_game_choice(&what_the_other_played)?;
    
        let round = game::Round::new(
            what_i_played,
            what_the_other_played,

        );
    
        return Ok(round);
    }

    enum ResultNeed {
        Loose,
        Draw,
        Win,
    }
    
    impl ResultNeed {
        fn to_game_choice(self, other_game_choice: &game::GameChoice) -> Result<game::GameChoice, &'static str> {
            let other_game_num = other_game_choice.to_num();
    
            match self {
                ResultNeed::Loose => Ok(game::GameChoice::from_num((other_game_num + 2) % 3)?),
                ResultNeed::Draw => Ok(game::GameChoice::from_num(other_game_num)?),
                ResultNeed::Win => Ok(game::GameChoice::from_num((3 + other_game_num + 1) % 3)?)
            }
        }
    }
    
    fn parse_expected_result(c: &str) -> Result<ResultNeed, &'static str> {
        match c {
            "X" => Ok(ResultNeed::Loose),
            "Y" => Ok(ResultNeed::Draw),
            "Z" => Ok(ResultNeed::Win),
            _ => Err("Invalid choice, expected X, Y or Z"),
        }
    }
    
}


mod game {
    #[derive(Debug)]
    pub enum GameChoice {
        Rock,
        Paper,
        Scissors,
    }
    
    impl GameChoice {
        pub fn to_num(&self) -> u32 {
            match self {
                GameChoice::Rock => 0,
                GameChoice::Paper => 1,
                GameChoice::Scissors => 2,
            }
        }
        pub fn from_num(n: u32) -> Result<GameChoice, &'static str> {
            match n {
                0 => Ok(GameChoice::Rock),
                1 => Ok(GameChoice::Paper),
                2 => Ok(GameChoice::Scissors),
                _ => Err("Invalid num entry, expected a number from 0 to 2")
            }
        }
     }
    
    #[derive(Debug)]
    pub struct Round {
        what_the_other_played: GameChoice,
        what_i_played: GameChoice,
    }
    
    impl Round {
        pub fn new(what_i_played: GameChoice, what_the_other_played: GameChoice) -> Round {
            Round{
                what_i_played,
                what_the_other_played
            }
        }

        fn base_score(&self) -> u32 {
            match self.what_i_played {
                GameChoice::Rock => 1,
                GameChoice::Paper => 2,
                GameChoice::Scissors => 3,
            }
        }
        fn compete_score(&self) -> u32 {
            let diff =
                (3 + self.what_the_other_played.to_num() - self.what_i_played.to_num()) % 3;
    
            match diff {
                0 => 3,
                1 => 0,
                2 => 6,
                _ => panic!("Unexpected diff value :( {}", diff)
            }
        }
    
        pub fn score(&self) -> u32 {
            return self.base_score() + self.compete_score();
        }
    }
    
    
    pub fn parse_other_player_choice(c: &str) -> Result<GameChoice, &str> {
        match c {
            "A" => Ok(GameChoice::Rock),
            "B" => Ok(GameChoice::Paper),
            "C" => Ok(GameChoice::Scissors),
            _ => Err("Invalid choice, expected A, B or C"),
        }
    }
}

