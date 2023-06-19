use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::parse;

use crate::parse::{Command, Listing};

#[derive(Debug, Clone)]
enum NodeData {
    File { size: i32 },
    Directory { children: RefCell<Vec<Rc<RefCell<Node>>>> },
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    data: NodeData,
    parent: RefCell<Weak<Node>>,
}

impl Node {
    fn new_root() -> Node {
        Node {
            name: String::from("/"),
            data: NodeData::Directory { children: RefCell::new(vec![]) },
            parent: RefCell::new(Weak::new()),
        }
    }

    fn new(name: String, data: NodeData, parent: &Rc<Node>) -> Node {
        Node {
            name,
            data,
            parent: RefCell::new(Rc::downgrade(parent)),
        }
    }

    fn size(&self) -> i32 {
        match &self.data {
            NodeData::File { size } => *size,
            NodeData::Directory { children } => {
                children.iter().map(|child| (**child).borrow().size()).sum()
            }
        }
    }

    fn add_child_from_data(&self, name: String, data: NodeData) {
        let node = Node{
            name,
            data,
            parent: RefCell::new(Weak::new()),
        };
        if let NodeData::Directory { children } = &self.data {
            *node.borrow().parent.borrow_mut() = Rc::downgrade(&Rc::clone(&self));
            children.borrow_mut().push(Rc::clone(&node));
        }
    }

    fn get_child_by_name(&self, name: &str) -> Option<Rc<RefCell<Node>>> {
        match &self.data {
            NodeData::File { .. } => None,
            NodeData::Directory { children } => {
                children.iter().find(|child| (**child).borrow().name == name)
            }
        }
    }
}

pub fn main(contents: String) -> Result<String, String> {
    let commands = parse::parse(contents)?;

    // Build a tree from the commands
    let root = Rc::new(Node::new_root());
    let mut current_dir = Rc::clone(&root);

    for command in commands {
        match command {
            Command::List { output } => {
                for listing in output {
                    match listing {
                        Listing::Directory { name } => {
                            current_dir.add_child_from_data(
                                name,
                                NodeData::Directory {
                                    children: RefCell::new(vec![]),
                                },
                            );
                        }
                        Listing::File { name, size } => {
                            current_dir.add_child_from_data(
                                name,
                                NodeData::File { size },
                            );
                        }
                    }
                }
            },
            Command::Cd { directory } => {
                // Find the directory
                let new_dir = current_dir.get_child_by_name(name);
            },
        }
    }

    Ok(String::from("Hello"))
}
