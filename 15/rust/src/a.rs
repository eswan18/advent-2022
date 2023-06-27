use crate::grid::Grid;

pub fn main(contents: String) -> Result<String, String> {
    let grid = Grid::build_from_text(&contents)?;
    print!("{}", grid);
    Ok(format!("a"))
}