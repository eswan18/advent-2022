use crate::heightmap::HeightMap;

pub fn main(contents: String) -> Result<String, String> {
    let map = HeightMap::build_from_str(&contents)?;
    let result = map.find_shortest_path_including_all_start_positions()?;
    Ok(format!("{}", result))
}