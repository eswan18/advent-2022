use crate::parse;

pub fn main(contents: String) -> Result<String, String> {
    let valves = parse::parse(contents)?;
    println!("{:?}", valves);
    Ok(String::from("TODO"))
}