mod a;
mod b;

use std::error::Error;

enum Part { A, B }

impl Part {
    fn from_str(s: &str) -> Result<Part, String> {
        match s.to_lowercase().as_str() {
            "a" => Ok(Part::A),
            "b" => Ok(Part::B),
            _ => Err(String::from("Invalid part")),
        }
    }
}

struct Args <'a> {
    part: Part,
    input_file: &'a str,
}

fn parse_args<'a>(raw_args: std::env::Args) -> Result<Args<'a>, String> {
    let raw_args: Vec<String> = raw_args.collect();
    if raw_args.len() != 3 {
        let message = format!("Usage: {} <a/b> <input>", raw_args[0]);
        return Err(message);
    }
    let part = Part::from_str(&raw_args[1])?;
    let input_file = raw_args[2].as_str();
    Ok(Args { part, input_file })
}

pub fn run(args: std::env::Args) -> Result<(), Box<dyn Error>> {
    let args = parse_args(args)?;

    let contents: String = std::fs::read_to_string(args.input_file)?;

    let answer: String = match args.part {
        A => a::main(contents)?,
        B => b::main(contents)?,
        _ => panic!("Invalid part"),
    };

    println!("Your answer is {}", answer);
    Ok(())
}