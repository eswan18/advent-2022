# [derive(Debug)]
enum ParsedLine {
    CommandLine(String),
    OutputLine(String),
}

impl ParsedLine {
    fn from_string(s: &str) -> ParsedLine {
        if s.starts_with("$ ") {
            let command = &s[2..];
            return ParsedLine::CommandLine(String::from(command))
        }
        ParsedLine::OutputLine(String::from(s))
    }
}

#[derive(Debug)]
pub enum Command {
    Cd{directory: String},
    List{output: Vec<String>},
}

pub fn parse(contents: String) -> Result<Vec<Command>, String> {
    let lines = contents.lines();
    let mut parsed_lines: Vec<ParsedLine> = lines
        .map(|line| ParsedLine::from_string(line) )
        .collect();

    let mut commands = vec![];
    while let Some(line) = parsed_lines.pop() {
        println!("hi");
        if let ParsedLine::CommandLine(command) = line {
            println!("hi2");
            let parts = command.split(" ").collect::<Vec<&str>>();
            let command = match parts[0] {
                "cd" => {
                    if parts.len() != 2 {
                        return Err(String::from("cd command must have exactly one argument"));
                    }
                    Command::Cd{directory: String::from(parts[1])}
                },
                "ls" => {
                    if parts.len() != 1 {
                        return Err(String::from("ls command must have no arguments"));
                    }
                    Command::List{output: vec![]}
                },
                _ => return Err(format!("Unknown command: {}", command)),
            };
            commands.push(command);
        } else {
            // If we've gotten here, we somehow found an output line that wasn't after a list command.
            println!("{:?}", line);
            return Err(String::from("Output line found without a list command"));
        }
    }
    println!("{:?}", commands);

    Ok(commands)
}

