use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::parse;

use crate::parse::{Command, Listing};

#[derive(Debug, Clone)]
enum NodeData {
    File { size: i32 },
    Directory,
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    value: NodeData,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

impl Node {
    fn new_root() -> Node {
        Node {
            name: String::from("/"),
            children: RefCell::new(vec![]),
            value: NodeData::Directory,
            parent: RefCell::new(Weak::new()),
        }
    }

    fn new(name: String, value: NodeData) -> Rc<Node> {
        Rc::new(Node {
            name,
            value,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        })
    }

    /*fn size(&self) -> i32 {
        match &self.data {
            NodeData::File { size } => *size,
            NodeData::Directory { children } => {
                children.iter().map(|child| (**child).borrow().size()).sum()
            }
        }
    }*/

    fn add_child(self: &Rc<Self>, child: Rc<Node>) {
        *child.parent.borrow_mut() = Rc::downgrade(self);
        self.children.borrow_mut().push(Rc::clone(&child));
    }

    fn find_child(&self, name: &str) -> Option<Rc<Node>> {
        for child in self.children.borrow().iter() {
            if child.name == name {
                return Some(Rc::clone(child));
            }
        }
        None
    }
}

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

    Ok(String::from("Hello"))
}
