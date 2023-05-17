use core::{str::Lines, panic};

enum Throw {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    You,
    Opponent,
    Tie,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <a/b> <input>", args[0]);
        std::process::exit(1);
    }
    let part = args[1].as_str();
    let input_file = args[2].as_str();
    let contents = std::fs::read_to_string(input_file).unwrap();
    let lines = contents.lines();
    let score = match part {
        "a" => do_a(lines),
        "b" => do_b(lines),
        _ => panic!("Invalid part"),
    };
    println!("Your score is {}", score.unwrap());
}

fn do_a(input: Lines) -> Result::<i32, String> {
    let turns: Vec<TurnA> = match input.map(|row| decode_row_a(row)).collect() {
        Ok(turns) => turns,
        Err(e) => return Err(e),
    };
    let scores = turns.iter().map(|turn| turn.score());
    Ok(scores.sum())
}

fn decode_row_a(row: &str) -> Result::<TurnA, String> {
    let parts = row.split(" ").collect::<Vec<_>>();

    let opponent = match parts[0] {
        "A" => Throw::Rock,
        "B" => Throw::Paper,
        "C" => Throw::Scissors,
        _ => return Err(("Invalid input").to_string()),
    };

    let you = match parts[1] {
        "X" => Throw::Rock,
        "Y" => Throw::Paper,
        "Z" => Throw::Scissors,
        _ => return Err(("Invalid input").to_string()),
    };

    Ok(TurnA { opponent, you })
}

struct TurnA {
    // The choices made by you and an opponent in a round of RPS
    opponent: Throw,
    you: Throw,
}

impl TurnA {
    fn outcome(&self) -> Outcome {
        match (&self.you, &self.opponent) {
            (Throw::Rock, Throw::Scissors) => Outcome::You,
            (Throw::Rock, Throw::Paper) => Outcome::Opponent,
            (Throw::Paper, Throw::Rock) => Outcome::You,
            (Throw::Paper, Throw::Scissors) => Outcome::Opponent,
            (Throw::Scissors, Throw::Paper) => Outcome::You,
            (Throw::Scissors, Throw::Rock) => Outcome::Opponent,
            _ => Outcome::Tie,
        }
    }

    fn score(&self) -> i32 {
        let outcome_score = match self.outcome() {
            Outcome::You => 6,
            Outcome::Opponent => 0,
            Outcome::Tie => 3,
        };
        let throw_score = match self.you {
            Throw::Rock => 1,
            Throw::Paper => 2,
            Throw::Scissors => 3,
        };
        outcome_score + throw_score
    }
}

fn do_b(input: Lines) -> Result::<i32, String> {
    let turns: Vec<TurnB> = match input.map(|row| decode_row_b(row)).collect() {
        Ok(turns) => turns,
        Err(e) => return Err(e),
    };

    let scores = turns.iter().map(|turn| turn.score());
    Ok(scores.sum())
}

fn decode_row_b(row: &str) -> Result::<TurnB, String> {
    let parts = row.split(" ").collect::<Vec<_>>();
    
    let opponent = match parts[0] {
        "A" => Throw::Rock,
        "B" => Throw::Paper,
        "C" => Throw::Scissors,
        _ => return Err(("Invalid input").to_string()),
    };

    let outcome = match parts[1] {
        "X" => Outcome::Opponent,
        "Y" => Outcome::Tie,
        "Z" => Outcome::You,
        _ => return Err(("Invalid input").to_string()),
    };

    Ok(TurnB::new(opponent, outcome))
}

struct TurnB {
    // The choices made by you and an opponent in a round of RPS
    you: Throw,
    outcome: Outcome,
}

impl TurnB {
    pub fn new(opponent: Throw, outcome: Outcome) -> TurnB {
        // Determine what you should throw
        let you = match (&opponent, &outcome) {
            (Throw::Rock, Outcome::You) => Throw::Paper,
            (Throw::Rock, Outcome::Opponent) => Throw::Scissors,
            (Throw::Rock, Outcome::Tie) => Throw::Rock,
            (Throw::Paper, Outcome::You) => Throw::Scissors,
            (Throw::Paper, Outcome::Opponent) => Throw::Rock,
            (Throw::Paper, Outcome::Tie) => Throw::Paper,
            (Throw::Scissors, Outcome::You) => Throw::Rock,
            (Throw::Scissors, Outcome::Opponent) => Throw::Paper,
            (Throw::Scissors, Outcome::Tie) => Throw::Scissors,
        };
        TurnB {
            outcome: outcome,
            you: you,
        }
    }

    fn score(&self) -> i32 {
        let outcome_score = match self.outcome {
            Outcome::You => 6,
            Outcome::Opponent => 0,
            Outcome::Tie => 3,
        };
        let throw_score = match self.you {
            Throw::Rock => 1,
            Throw::Paper => 2,
            Throw::Scissors => 3,
        };
        outcome_score + throw_score
    }
}