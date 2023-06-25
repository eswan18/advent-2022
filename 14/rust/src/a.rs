use crate::cave_state::CaveState;

pub fn main(contents: String) -> Result<String, String> {
    let mut state = CaveState::build_from_text(&contents)?;
    state.drop_sand_until_in_abyss();
    
    Ok((state.n_dropped - 1).to_string())
}