#[cfg(test)]
mod tests;
mod error;
mod node;
mod node_tree;

use error::*;
use node::{Node, NodeKind};
use node_tree::*;

pub fn parse(input : &str) -> Result<NodeTree, ParseError> {
    let mut in_tag = false;
    let mut start_tag = false;
    let mut end_tag = false;
    let mut buffer : Option<char> = None;
    let mut current : String = String::new();
    let mut inner : String = String::new();
    let mut tree = NodeTree::new();
    let mut node = Node::new();

    for c in String::from(input).chars() {
        match c {
            '<' => {
                if in_tag {
                    node.innner = inner;
                    inner = String::new();
                }
                start_tag = true;
                current = String::new();
            },
            '>' => {
                if start_tag {
                    in_tag = true;
                    node.meta = Some(current);
                    current = String::new();
                }
                start_tag = false;

                if end_tag {
                    tree.add_child(node);
                    node = Node::new();
                    
                    in_tag = false;
                    current = String::new();
                }
            },
            '/' => {
                if let Some(buf) = buffer {
                    if buf == '<'  && start_tag {
                        end_tag = true;
                        in_tag = false;
                        start_tag = false;
                    } else {
                        current.push(c);
                    }
                }
            },
            _ => {
                current.push(c);
            }
        }

        buffer = Some(c);
    }

    Ok(tree)
}

fn parse_inner_tag (input : String) {
    println!("Tag : {}", input);
}