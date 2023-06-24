use crate::instruction::Instruction;
use crate::cpu::CpuState;

const LINE_LENGTH: usize = 40;


pub fn main(contents: String) -> Result<String, String> {
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
    let mut instructions = Instruction::parse_lines(lines)?;

    let mut state = CpuState::new();
    state.queue_instructions(&mut instructions);

    let mut text = String::from("\n");
    loop {
        let pixel = (state.starting_cycle - 1) % LINE_LENGTH;
        let sprite_pos = state.value;

        if (pixel as i32 - sprite_pos).abs() <= 1 {
            text.push_str("#");
        } else {
            text.push_str(" ");
        }
        if (pixel + 1) % LINE_LENGTH == 0 {
            text.push_str("\n");
        } 
        if let Err(_) = state.tick() {
            break;
        }
    }

    Ok(text)
}