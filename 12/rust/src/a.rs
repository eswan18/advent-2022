use crate::heightmap::HeightMap;

pub fn main(contents: String) -> Result<String, String> {
    let map = HeightMap::build_from_str(&contents)?;
    let result = map.find_shortest_path()?;
    Ok(format!("{}", result))
}