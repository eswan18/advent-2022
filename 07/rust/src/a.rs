use std::cell::RefCell;

use crate::parse;

use crate::parse::{Command, Listing};

#[derive(Debug, Clone)]
enum NodeData {
    File(File),
    Directory,
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    data: NodeData,
    children: Vec<RefCell<Node>>,
}

impl Node {
    fn new_root(name: String) -> Node {
        Node {
            name,
            data: NodeData::Directory,
            children: vec![],
        }
    }

    fn new_from_data(name: String, data: NodeData) -> Node {
        Node {
            name,
            data,
            children: vec![],
        }
    }

    fn size(&self) -> i32 {
        match &self.data {
            NodeData::File(file) => file.size,
            NodeData::Directory => {
                self.children.iter().map(|child| {
                    let child = (*child).borrow();
                    child.size()
                }).sum()
            }
        }
    }

    fn add_child_from_ref(&mut self, child: RefCell<Node>) {
        self.children.push(child);
    }

    fn add_child_from_data(&mut self, name: String, child: NodeData) {
        let node = Node::new_from_data(name, child);
        self.add_child_from_ref(RefCell::new(node));
    }

    fn update_from_listing(&mut self, l: Listing) {
        match l {
            Listing::Directory { name } => {
                let already_exists = self.children.iter().any(|child| { child.borrow().name == name });
                if already_exists {
                    return;
                }
                let child = NodeData::Directory;
                self.add_child_from_data(name, child);
            }
            Listing::File { name, size } => {
                let already_exists = self.children.iter().any(|child| { child.borrow().name == name });
                if already_exists {
                    return;
                }
                let name_copy = name.clone();
                let file = File{ name, size };
                self.add_child_from_data(name_copy, NodeData::File(file));
            }
        }
    }
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: i32,
}

pub fn main(contents: String) -> Result<String, String> {
    let commands = parse::parse(contents)?;

    // Build a tree from the commands
    let root = RefCell::new(Node::new_root(String::from("/")));
    let mut dir_stack = vec![root];

    for command in commands {
        let mut current_dir = dir_stack[dir_stack.len() - 1].borrow_mut();
        match command {
            Command::Cd { directory } => {
                let new_dir = current_dir
                    .children.iter()
                    .find(|child| child.borrow().name == directory)
                    .ok_or(String::from("Directory not found"))?;
                dir_stack.push(*new_dir);
            },
            Command::List { output } => {
                for listing in output {
                    current_dir.update_from_listing(listing);
                }
            },
        }
    }

    Ok(String::from("Hello"))
}

