use crate::parse;
use crate::valve::{Valve, self};

pub fn main(contents: String) -> Result<String, String> {
    let valves = parse::parse(contents)?;
    let distance_matrix = valve::compute_distance_matrix(&valves);
    println!("{:?}", distance_matrix);
    println!("AA->BB {:?}", distance_matrix.get("AA").unwrap().get("BB").unwrap());
    Ok(String::from("TODO"))
}