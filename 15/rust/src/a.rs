use crate::grid::Grid;

const ROW_OF_INTEREST: i32 = 2_000_000;

pub fn main(contents: String) -> Result<String, String> {
    let grid = Grid::build_from_text(&contents)?;
    let excluded_of_interest = grid.excluded_count_in_row(ROW_OF_INTEREST);
    Ok(excluded_of_interest.to_string())
}
