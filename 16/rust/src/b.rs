use crate::parse;
use crate::valve::DistanceMatrix;

const STEPS: usize = 26;

pub fn main(contents: String) -> Result<String, String> {
    let valves = parse::parse(contents)?;
    let distance_matrix = DistanceMatrix::new(valves);
    let distance_matrix = distance_matrix.with_valves_removed();
    let max_flow = distance_matrix.maximize_flow_2_paths(STEPS);
    Ok(max_flow.to_string())
}