use crate::grid::{Grid, Position};

const MAX_COORD: i32 = 4000000;

pub fn main(contents: String) -> Result<String, String> {
    let grid = Grid::build_from_text(&contents)?;
    let open_spaces: Vec<Position> = grid.open_spaces_with_max_coord(MAX_COORD);
    println!("Found {} total open spaces", open_spaces.len());
    assert!(open_spaces.len() == 1);
    let open_space = &open_spaces[0];
    let tuning_freq = open_space.x * 4000000 + open_space.y;
    Ok(tuning_freq.to_string())
}
