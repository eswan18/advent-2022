mod a;
mod b;
mod cargo;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <a/b> <input>", args[0]);
        std::process::exit(1);
    }

    let part: &str = args[1].as_str();
    let input_file: &str = args[2].as_str();
    let contents: String = std::fs::read_to_string(input_file)?;

    let answer: String = match part {
        "a" => a::main(contents)?,
        "b" => b::main(contents)?,
        _ => panic!("Invalid part"),
    };

    println!("Your answer is {}", answer);

    Ok(())
}