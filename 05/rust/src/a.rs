use crate::cargo;

pub fn main(input: String) -> Result<String, String> {
    let state = cargo::parse_input(input)?;
    let mut stacks = state.stacks;
    let instructions = state.instructions;

    for instr in instructions {
        // Execute each instruction N times.
        for _ in 0..instr.count {
            let crate_ = stacks[instr.from as usize].pop().unwrap();
            stacks[instr.to as usize].push(crate_);
        }
    }

    let s: String = stacks.iter().map(|s| s.peek().unwrap().char).collect();
    Ok(s)
}