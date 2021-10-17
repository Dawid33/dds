use crate::{error::ParseError, node::*};

pub struct NodeTree<'a> {
    pub root : Node<'a>,
    current : (i32,i32),
}

impl<'a> NodeTree<'a> {
    pub fn new() -> Self {
        Self {
            root : Node::new(),
            current : (0,0),
        }
    }
    pub fn add_child(&mut self, node : Node<'a>) {
        self.root.add_child(node);
    }
    pub fn make_parent_current(&mut self) -> Result<(), ParseError> {
        if self.current.0 == 0 {
            return Err(ParseError::NodeNotExist);
        }
        
        self.current.0 -= 1;
        Ok(())
    }
    pub fn current_node(&self) -> &'a Node{
        let node = &(self).root;
        node
    }
}