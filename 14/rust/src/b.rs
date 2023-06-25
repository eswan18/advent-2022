use crate::cave_state::CaveState;

pub fn main(contents: String) -> Result<String, String> {
    let mut state = CaveState::build_from_text(&contents)?;
    // Add an additional line of blockers at the bottom of the cave.
    state.add_cave_bottom();
    state.drop_sand_until_blocked();
    Ok(state.n_dropped.to_string())
}