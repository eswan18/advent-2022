use std::rc::Rc;

use crate::parse;
use crate::node::{Node, NodeData};

use crate::parse::{Command, Listing};

pub fn main(contents: String) -> Result<String, String> {
    let commands = parse::parse(contents)?;

    // Build a tree from the commands
    let root = Node::new_root();
    let root_ref = Rc::new(root);
    let mut current_dir = Rc::clone(&root_ref);

    for command in commands {
        println!("Executing {:?}", command);

        match command {
            Command::List { output } => {
                for listing in output {
                    let child = match listing {
                        Listing::Directory { name } => Node::new(name, NodeData::Directory),
                        Listing::File { name, size } => Node::new(name, NodeData::File { size }),
                    };
                    Rc::clone(&current_dir).add_child(child.clone());
                    println!("Added child {:?}", child);
                }
            },

            Command::Cd { directory } => {
                match directory.as_str() {
                    "/" => {
                        current_dir = Rc::clone(&root_ref);
                        println!("Navigated to ROOT");
                        continue;
                    },
                    ".." => {
                        // navigate up.
                        let maybe_parent = current_dir.parent.borrow().upgrade();
                        if let Some(parent) = maybe_parent {
                            current_dir = parent;
                            println!("Navigated UP into {:?}", current_dir);
                            continue;
                        } else {
                            panic!("Can't navigate above a root directory");
                        };
                    },
                    _ => {
                        // Find the directory
                        current_dir = current_dir.find_child(&directory).ok_or_else(|| String::from("Directory not found"))?;
                        println!("Navigated into {:?}", current_dir);
                    },
                }
            },
        }
    }

    let size = root_ref.size();
    Ok(size.to_string())
}
