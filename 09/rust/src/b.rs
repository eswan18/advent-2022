use crate::rope::{Instruction, MultiRope};

const N_KNOTS: usize = 9;

pub fn main(contents: String) -> Result<String, String> {
    let instructions = Instruction::build_from_file_contents(contents)?;
    let mut rope = MultiRope::new_at_origin(N_KNOTS);
    for i in instructions {
        rope.do_instruction(i);
    }
    Ok(rope.get_count_uniq_tail_spaces().to_string())
}