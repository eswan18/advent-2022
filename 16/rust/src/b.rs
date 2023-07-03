use crate::parse;
use crate::distance_matrix::DistanceMatrix;
use crate::state::GameState;

const STEPS: usize = 26;

pub fn main(contents: String) -> Result<String, String> {
    let valves = parse::parse(contents)?;
    let distance_matrix = DistanceMatrix::new(valves);
    let distance_matrix = distance_matrix.with_valves_removed();

    let state = GameState::new(2, &distance_matrix, STEPS);
    let answer = state.maximize_flow();
    Ok(answer.to_string())
}