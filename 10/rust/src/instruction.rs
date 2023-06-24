pub enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    pub fn parse_lines(lines: Vec<String>) -> Result<Vec<Instruction>, String> {
        let mut instructions: Vec<Instruction> = Vec::new();
        for line in lines {
            let instruction = Instruction::parse_line(line)?;
            instructions.push(instruction);
        }
        Ok(instructions)
    }

    pub fn parse_line(line: String) -> Result<Instruction, String> {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        match parts[..] {
            ["noop"] => Ok(Instruction::Noop),
            ["addx", x] => {
                let x = x.parse::<i32>().map_err(|e| e.to_string())?;
                Ok(Instruction::AddX(x))
            }
            _ => Err(format!("Invalid instruction: {}", line)),
        }
    }

    pub fn cycles(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::AddX(_) => 2,
        }
    }

    pub fn transform_value(&self, value: i32) -> i32 {
        match self {
            Instruction::Noop => {
                value
            },
            Instruction::AddX(x) => {
                value + x
            },
        }
    }
}