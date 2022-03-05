use std::num::NonZeroUsize;

pub struct Tree<T> {
    nodes : Vec<Node<T>>,
}

struct Node<T> {
    parent: Option<NodeId>,
    prev_sibling: Option<NodeId>,
    next_sibling: Option<NodeId>,
    children: Option<Vec<NodeId>>,
    value : T,
}

pub struct NodeId(NonZeroUsize);

impl<T> Tree<T> {
    pub fn new(root : T) -> Tree<T> {
        Tree {
            nodes: vec![Node::new(root)],
        }
    }
}

impl<T> Node<T> {
    pub fn new(value : T) -> Node<T> {
        Node {
            parent: None,
            prev_sibling: None,
            next_sibling: None,
            children: None,
            value,
        }
    }
}

