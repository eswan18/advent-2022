use crate::cargo;


pub fn main(input: String) -> Result<String, String> {
    let state = cargo::parse_input(input)?;
    let mut stacks = state.stacks;
    let instructions = state.instructions;

    for instr in instructions {
        let crates = stacks[instr.from as usize]
            .popn(instr.count as usize)
            .ok_or("Not enough crates")?;
        for c in crates {
            stacks[instr.to as usize].push(c);
        }
    }

    let s: String = stacks.iter().map(|s| s.peek().unwrap().char).collect();
    Ok(s)
}