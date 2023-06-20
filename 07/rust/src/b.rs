use crate::node::{Node, NodeData};
use crate::parse;

const TOTAL_SPACE: i32 = 70000000;
const NEEDED_SPACE: i32 = 30000000;

pub fn main(contents: String) -> Result<String, String> {
    let commands = parse::parse(contents)?;
    let tree = Node::new_from_commands(commands)?;

    let used_space = tree.size();
    let free_space = TOTAL_SPACE - used_space;
    let deficit = NEEDED_SPACE - free_space;

    // Find all directories big enough to overcome the deficit
    let descendants = tree.descendants();
    let big_dirs: Vec<(&String, i32)> = descendants
        .iter()
        .filter(|x| match x.value {
            NodeData::Directory => true,
            _ => false,
        })
        .filter(|x| x.size() >= deficit)
        .map(|x| (&x.name, x.size()))
        .collect();

    // Find the smallest of these
    let smallest_big_dir = big_dirs.iter().min_by_key(|(_, size)| size).unwrap();

    Ok(smallest_big_dir.1.to_string())
}
