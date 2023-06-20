use crate::forest::Forest;

pub fn main(contents: String) -> Result<String, String> {
    let forest = Forest::new_from_text(contents)?;
    let visible_trees = forest.visible_positions();
    let n_visible = visible_trees.len();
    Ok(n_visible.to_string())
}