use crate::parse;
use crate::distance_matrix::DistanceMatrix;
use crate::state::GameState;
use std::rc::Rc;

const STEPS: usize = 30;

pub fn main(contents: String) -> Result<String, String> {
    let valves = parse::parse(contents)?;
    let distance_matrix = DistanceMatrix::new(valves);
    let distance_matrix = distance_matrix.with_valves_removed();

    let state = GameState::new(1, Rc::new(distance_matrix), STEPS);
    let answer = state.maximize_flow();
    Ok(answer.to_string())
}