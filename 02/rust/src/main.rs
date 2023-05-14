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

struct Turn {
    // The choices made by you and an opponent in a round of RPS
    opponent: Throw,
    you: Throw,
}

impl Turn {
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
    let turns: Vec<Turn> = match input.map(|row| decode_row_a(row)).collect() {
        Ok(turns) => turns,
        Err(e) => return Err(e),
    };
    let scores = turns.iter().map(|turn| turn.score());
    Ok(scores.sum())
}

fn do_b(_input: Lines) -> Result::<i32, String> {
    panic!("Not implemented")
}

fn decode_row_a(row: &str) -> Result::<Turn, String> {
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

    Ok(Turn { opponent, you })
}