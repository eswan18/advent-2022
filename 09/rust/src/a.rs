use crate::rope::{Instruction, Rope};

pub fn main(contents: String) -> Result<String, String> {
    let instructions = Instruction::build_from_file_contents(contents)?;
    let mut rope = Rope::new_at_origin();
    for i in instructions {
        rope.do_instruction(i);
    }
    Ok("hi".to_string())
}