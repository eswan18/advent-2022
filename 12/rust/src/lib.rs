mod a;
mod b;
mod heightmap;

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

struct Args {
    part: Part,
    input_file: String,
}

fn parse_args(raw_args: std::env::Args) -> Result<Args, String> {
    let raw_args: Vec<String> = raw_args.collect();
    if raw_args.len() != 3 {
        let message = format!("Usage: {} <a/b> <input>", raw_args[0]);
        return Err(message);
    }
    let part = Part::from_str(&raw_args[1])?;
    let input_file = raw_args[2].clone();
    Ok(Args { part, input_file })
}

pub fn run(args: std::env::Args) -> Result<(), String> {
    let args = parse_args(args)?;

    let contents = std::fs::read_to_string(args.input_file)
        .map_err(|e: std::io::Error| e.to_string())?;

    let answer: String = match args.part {
        Part::A => a::main(contents)?,
        Part::B => b::main(contents)?,
    };

    println!("Your answer is {}", answer);
    Ok(())
}