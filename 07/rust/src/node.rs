use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::iter::IntoIterator;

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

