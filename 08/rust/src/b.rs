use crate::forest::Forest;

pub fn main(contents: String) -> Result<String, String> {
    let forest = Forest::new_from_text(contents)?;
    let (rows, cols) = forest.dimensions();

    let mut best_score = -1;
    for x in 0..cols {
        for y in 0..rows {
            let current_score = forest.scenic_score(x, y)? as i32;
            if current_score > best_score {
                best_score = current_score;
            }
        }
    }

    Ok(best_score.to_string())
}