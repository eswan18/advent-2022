use crate::parse;
use crate::distance_matrix::DistanceMatrix;

const STEPS: usize = 30;

pub fn main(contents: String) -> Result<String, String> {
    let valves = parse::parse(contents)?;
    let distance_matrix = DistanceMatrix::new(valves);
    let distance_matrix = distance_matrix.with_valves_removed();
    let max_flow = distance_matrix.maximize_flow(STEPS);
    Ok(max_flow.to_string())
}