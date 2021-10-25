use crate::{error::ParseError, node::*};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::slice::SliceIndex;
use std::sync::{Mutex, RwLock};

/*
pub enum SearchTerm {
    uuid(i32),
}

pub struct SearchHandler {
    thread_handle : std::thread::JoinHandle<Option<Node>>,
    search_term : SearchTerm,
}
*/

#[derive(Clone)]
pub struct NodeTree {
    root : Rc<RefCell<Node>>,
}

#[derive(Clone)]
pub struct NodeTreeBuilder {
    tree : NodeTree,
    pub current_node : Rc<RefCell<Node>>,
}

impl NodeTreeBuilder {
    pub fn new() -> Self {
        let tree = NodeTree::new();
        Self {
            current_node : tree.root.clone(),
            tree,
        }
    }

    pub fn build(self) -> NodeTree {
        return self.tree;
    }

    pub fn push_node(&mut self, node : Node) {
        (*self.current_node).borrow_mut().add_child(Rc::new(RefCell::new(node)));
    }

    pub fn append(&mut self, other : Self) {
        self.current_node.deref().borrow_mut().add_child(other.tree.root);
    }
}

impl NodeTree {
    pub fn new() -> Self {
        Self {
            root : Rc::new(RefCell::new(Node::new())),
        }
    }

    pub fn print_nodes(&self) -> Option<Rc<RefCell<Node>>> {
        let root_node  = &*self.root;
        if let Some(children) = &root_node.borrow().children {
            let mut cnt = 0;
            for child in children {
                println!("Node {}: {}", cnt, (**child).borrow().inner);
                cnt += 1;
            }
        }
        None
    }
}