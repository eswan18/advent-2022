use crate::node::{Node, NodeData};
use crate::parse;

const MAX_SIZE: i32 = 100000;

pub fn main(contents: String) -> Result<String, String> {
    let commands = parse::parse(contents)?;

    // Build a tree from the commands
    let tree = Node::new_from_commands(commands)?;

    // Find all directories with a size of at most MAX_SIZE
    let mut small_dirs = Vec::new();

    let descendants = tree.descendants();
    let dir_descendants = descendants.iter().filter(|x| match x.value {
        NodeData::Directory => true,
        _ => false,
    });
    for dir in dir_descendants {
        let dir_size = dir.size();
        if dir_size <= MAX_SIZE {
            small_dirs.push((dir, dir_size));
        }
    }

    let total_size_of_small_dirs = small_dirs.iter().map(|(_, size)| size).sum::<i32>();
    Ok(total_size_of_small_dirs.to_string())
}
