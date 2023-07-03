use crate::parse;
use crate::valve::{Valve, DistanceMatrix};

pub fn main(contents: String) -> Result<String, String> {
    let valves = parse::parse(contents)?;
    let distance_matrix = DistanceMatrix::new(valves);
    let distance_matrix = distance_matrix.with_valves_removed();
    println!("{}", distance_matrix);
    println!("AA->BB {:?}", distance_matrix.distance("AA", "BB"));
    Ok(String::from("TODO"))
}