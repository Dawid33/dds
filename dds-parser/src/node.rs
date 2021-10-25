use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub enum NodeKind {
    Div,
    None
}

pub struct Node {
    pub kind : NodeKind,
    pub inner : String,
    pub children : Option<Vec<Rc<RefCell<Node>>>>,
    pub parent : Option<Weak<RefCell<Node>>>,
}

impl Node{
    pub fn new() -> Node {
        Node {
            kind : NodeKind::None,
            inner : String::new(),
            children : None,
            parent : None,
        }
    }

    pub fn add_child(&mut self, node : Rc<RefCell<Node>>) {
        if let Some(ref mut children) = &mut self.children {
            children.push(node);
        } else {
            self.children = Some(vec![node]);
        }
    }

    pub fn get_children(&self) {
        
    }
}