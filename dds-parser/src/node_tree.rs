use crate::{error::ParseError, node::*};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::slice::SliceIndex;
use std::sync::{Mutex, RwLock};

/*
pub enum SearchTerm {
    uuid(i32),
}

pub struct SearchHandler<T> {
    thread_handle : std::thread::JoinHandle<Option<Node<T>>>,
    search_term : SearchTerm,
}
*/

pub struct NodeTree<T> {
    root : Rc<RefCell<Node<T>>>,
}

pub struct NodeTreeBuilder<T> {
    tree : NodeTree<T>,
    current_node : Rc<RefCell<Node<T>>>,
}

impl<T> NodeTreeBuilder<T> {
    pub fn new() -> Self {
        let tree = NodeTree::new();
        Self {
            current_node : tree.root.clone(),
            tree,    
        }
    }

    pub fn push_node(&mut self, node : Node<T>) {
        (*self.current_node).borrow_mut().add_child(node);
    }

    pub fn build(self) -> NodeTree<T> {
        return self.tree;
    }
}

impl<T> NodeTree<T> {
    pub fn new() -> Self {
        Self {
            root : Rc::new(RefCell::new(Node::new(0))),
        }
    }

    pub fn get_node_by_id(&self, id : i32) -> Option<Rc<RefCell<Node<T>>>> {
        let root_node  = &*self.root;
        if let Some(children) = &root_node.borrow().children {
            for child in children {
                if (**child).borrow().id == id {
                    return Some(child.clone());
                }
            }
        }
        None
    }
}