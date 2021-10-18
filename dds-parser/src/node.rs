use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub enum NodeKind {
    Div,
    None
}

pub struct Node<T> {
    pub kind : NodeKind,
    pub id : i32,
    pub meta : Option<String>,
    pub inner : String,
    pub children : Option<Vec<Rc<RefCell<Node<T>>>>>,
    pub parent : Option<Weak<RefCell<Node<T>>>>,
}

impl<T> Node<T>{
    pub fn new(id : i32) -> Node<T> {
        Node {
            kind : NodeKind::None,
            id : id,
            meta : None,
            inner : String::new(),
            children : None,
            parent : None,
        }
    }

    pub fn add_child(&mut self, node : Node<T>) {
        let new_child = Rc::new(RefCell::new(node));
        if let Some(ref mut children) = &mut self.children {
            children.push(new_child);
        } else {
            self.children = Some(vec![new_child]);
        }
    }

    pub fn get_children(&self) {
        
    }
}