use std::fmt;
use regex::Regex;

pub struct Crate {
    pub char: char,
}

impl Crate {
    pub fn new(char: char) -> Self {
        Self { char }
    }
}

impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.char)
    }
}

pub struct Stack {
    crates: Vec<Crate>,
}

impl Stack {
    pub fn new() -> Self {
        Self { crates: Vec::new() }
    }

    pub fn push(&mut self, crate_: Crate) {
        self.crates.push(crate_);
    }

    pub fn pop(&mut self) -> Option<Crate> {
        self.crates.pop()
    }

    pub fn peek(&self) -> Option<&Crate> {
        self.crates.last()
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for crate_ in self.crates.iter() {
            s.push(crate_.char);
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub from: u32,
    pub to: u32,
    pub count: u32,
}

impl Instruction {
    pub fn parse(line: &str) -> Self {
        // instructions come in the form "move 1 from 1 to 2".
        let line = line.trim();
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let captures = re.captures(line).unwrap();
        Instruction {
            count: captures[1].parse::<u32>().unwrap(),
            from: captures[2].parse::<u32>().unwrap() - 1,
            to: captures[3].parse::<u32>().unwrap() - 1,
        }
    }
}

pub struct GameState {
    pub stacks: Vec<Stack>,
    pub instructions: Vec<Instruction>,
}

pub fn parse_input(input: String) -> Result<GameState, String> {
    // Break the input into two chunks based on the blank line.
    let chunks: Vec<&str> = input.split("\n\n").collect();

    let stacks = parse_stacks(chunks[0])?;
    let instructions = parse_instructions(chunks[1])?;

    Ok(GameState { stacks, instructions })
}

fn parse_stacks(input: &str) -> Result<Vec<Stack>, String> {
    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();
    let n_lines: usize = lines.len();

    let number_line: &str = lines[n_lines - 1];

    let crate_lines = &lines[0..n_lines - 1];

    let column_count: usize = number_line
        .split_whitespace()
        .last()
        .ok_or(String::from("No number"))?
        .parse::<usize>()
        .expect("Not a number");

    let mut stacks: Vec<Stack> = Vec::new();
    for _ in 0..column_count {
        stacks.push(Stack::new());
    }

    // Start at the bottom of each column and add crates to the stacks.
    for line in crate_lines.iter().rev() {
        let chars: Vec<char> = line.chars().collect::<Vec<char>>();
        for i in 0..column_count {
            // Figure out where in the line we should look for this column's crate.
            let line_pos: usize = 1 + i * 4; 
            let char = chars.get(line_pos).unwrap_or(&' ');

            if *char == ' ' { continue };
            stacks[i].push(Crate::new(*char));
        }
    }
    Ok(stacks)
}

fn parse_instructions(input: &str) -> Result<Vec<Instruction>, String> {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line: &str| Instruction::parse(line))
        .collect();
    Ok(instructions)
}