# [derive(Debug)]
enum ParsedLine {
    Command(ParsedCommand),
    Output(String),
}

impl ParsedLine {
    fn from_string(s: &str) -> Result<ParsedLine, String> {
        if s.starts_with("$ ") {
            let command = &s[2..];
            return Ok(ParsedLine::Command(ParsedCommand::from_string(command)?));
        }
        Ok(ParsedLine::Output(String::from(s)))
    }
}

# [derive(Debug)]
enum ParsedCommand {
    Cd(String),
    List,
}

impl ParsedCommand {
    fn from_string(s: &str) -> Result<ParsedCommand, String> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        return match parts[0] {
            "cd" => {
                if parts.len() < 2 {
                    return Err(String::from("cd requires an argument"));
                }
                Ok(ParsedCommand::Cd(String::from(parts[1])))
            }
            "ls" => Ok(ParsedCommand::List),
            _ => Err(String::from("Invalid command")),
        }
    }
}

enum FullCommand {
    Cd(String),
    List(CommandOutput),
}

struct CommandOutput {
    lines: Vec<String>,
}


pub fn main(contents: String) -> Result<String, String> {
    let lines = contents.lines();
    let parsed_lines: Vec<ParsedLine> = lines
        .map(|line| {
            ParsedLine::from_string(line)
        })
        .collect::<Result<Vec<ParsedLine>, String>>()?;
    
    println!("{:?}", parsed_lines);
    Ok(String::from("Hello"))
}