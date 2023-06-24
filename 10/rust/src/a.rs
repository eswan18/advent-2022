use crate::instruction::Instruction;
use crate::cpu::CpuState;

pub fn main(contents: String) -> Result<String, String> {
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    let mut instructions = Instruction::parse_lines(lines)?;

    let cycles_of_note: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    let mut state = CpuState::new();
    state.queue_instructions(&mut instructions);

    let mut total_strength = 0;
    loop {
        if state.starting_cycle > *cycles_of_note.last().unwrap() {
            break;
        }
        println!("Cycle {}: {}  =>  {}", &state.starting_cycle, &state.value, &state.signal_strength());
        if cycles_of_note.contains(&state.starting_cycle) {
            total_strength += state.signal_strength();
        }
        if let Err(_) = state.tick() {
            break;
        }
    }

    Ok(total_strength.to_string())
}