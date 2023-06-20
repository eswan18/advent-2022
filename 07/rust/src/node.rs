use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::parse::{Command, Listing};

#[derive(Debug, Clone)]
pub enum NodeData {
    File { size: i32 },
    Directory,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub value: NodeData,
    children: RefCell<Vec<Rc<Node>>>,
    pub parent: RefCell<Weak<Node>>,
}

impl Node {
    pub fn new_from_commands(commands: Vec<Command>) -> Result<Node, String> {
        let root = Node::new_root();
        let root_ref = Rc::new(root);
        let mut current_dir = Rc::clone(&root_ref);

        for command in commands {
            match command {
                Command::List { output } => {
                    for listing in output {
                        let child = match listing {
                            Listing::Directory { name } => Node::new(name, NodeData::Directory),
                            Listing::File { name, size } => Node::new(name, NodeData::File { size }),
                        };
                        Rc::clone(&current_dir).add_child(child.clone());
                    }
                },

                Command::Cd { directory } => {
                    match directory.as_str() {
                        "/" => {
                            current_dir = Rc::clone(&root_ref);
                            continue;
                        },
                        ".." => {
                            // navigate up.
                            let maybe_parent = current_dir.parent.borrow().upgrade();
                            if let Some(parent) = maybe_parent {
                                current_dir = parent;
                                continue;
                            } else {
                                panic!("Can't navigate above a root directory");
                            };
                        },
                        _ => {
                            // Find the directory
                            current_dir = current_dir.find_child(&directory).ok_or_else(|| String::from("Directory not found"))?;
                        },
                    }
                },
            }
        }

        // Get the underlying value out of root_ref
        let root = Rc::try_unwrap(root_ref).map_err(|_| String::from("Could not unwrap root"))?;
        Ok(root)
    }

    pub fn new_root() -> Node {
        Node {
            name: String::from("/"),
            children: RefCell::new(vec![]),
            value: NodeData::Directory,
            parent: RefCell::new(Weak::new()),
        }
    }

    pub fn new(name: String, value: NodeData) -> Rc<Node> {
        Rc::new(Node {
            name,
            value,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        })
    }

    pub fn size(&self) -> i32 {
        match &self.value {
            NodeData::File { size } => *size,
            NodeData::Directory => {
                self.children.borrow().iter().map(|child| child.size()).sum()
            }
        }
    }

    pub fn add_child(self: &Rc<Self>, child: Rc<Node>) {
        *child.parent.borrow_mut() = Rc::downgrade(self);
        self.children.borrow_mut().push(Rc::clone(&child));
    }

    pub fn find_child(&self, name: &str) -> Option<Rc<Node>> {
        for child in self.children.borrow().iter() {
            if child.name == name {
                return Some(Rc::clone(child));
            }
        }
        None
    }

    pub fn children(&self) -> Vec<Rc<Node>> {
        self.children.borrow().clone()
    }

    pub fn descendants(&self) -> Vec<Rc<Node>> {
        let mut descendants = Vec::new();
        let mut to_visit = self.children.borrow().clone();

        while let Some(node) = to_visit.pop() {
            descendants.push(node.clone());
            let children = node.children();
            to_visit.extend(children);
        }

        descendants
    }

}

