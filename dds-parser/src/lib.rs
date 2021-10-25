#![allow(unused)]
mod tests;
mod error;
mod node;
mod node_tree;
mod tag;

use std::{borrow::Borrow, cell::RefCell, ops::Deref, rc::Rc, str::Chars};

use error::*;
use node::{Node, NodeKind};
use node_tree::*;
use tag::Tag;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Position {
    OpeningTag,
    ClosingTag,
    InnerContents,
    UndefinedTag,
    Undefined,
    Finished,
}

#[derive(Clone, Copy)]
pub struct ParseState {
    current_pos : Position,
    previous : char,
}

impl ParseState {
    pub fn new() -> Self {
        Self {
            current_pos : Position::InnerContents,
            previous : ' ',
        }
    }
}

pub fn parse_tag<'a,'b>(mut input : &'a mut Chars<'b>, mut state : ParseState) -> Result<(NodeTreeBuilder, ParseState), ParseError> {
    let mut tree = NodeTreeBuilder::new();
    let mut node = Node::new();

    while (state.current_pos != Position::Finished) {
        for c in &mut *input {
            match c {
                //Entering either opening tag or closing tag.
                '<' => {
                    state.current_pos = match(state.current_pos) {
                        Position::InnerContents => Position::UndefinedTag,
                        // '<' cannot exist if still inside a closing tag, error.
                        Position::ClosingTag => panic!(),
                        // Same error as if we are inside a closing tag.
                        Position::OpeningTag => panic!(),
                        // Should never happen.
                        Position::UndefinedTag => panic!(),
                        // Should never happen.
                        Position::Undefined => panic!(),
                        // Should never happen.
                        Position::Finished => Position::Finished,
                    }
                }
                //Leaving either opening tag or closing tag.
                '>' => {
                    state.current_pos = match(state.current_pos) {
                        // Moving inside node.
                        Position::OpeningTag => Position::InnerContents,
                        // Leaving tag, should return node.
                        Position::ClosingTag => Position::Finished,
                        // Cannot be inside inner contents.
                        Position::InnerContents => panic!(),
                        Position::Undefined => panic!(),
                        Position::UndefinedTag => panic!(),
                        Position::Finished => Position::Finished,
                    }
                },
                _ => {
                    if state.previous == '<' && state.current_pos == Position::UndefinedTag {
                        match(c) {
                            '/' => {
                                state.current_pos = Position::ClosingTag;
                            },
                            _ => {
                                state.current_pos = Position::OpeningTag;
                                
                                let (new_tree, new_state) = parse_tag(&mut input, state)?;
                                let x = new_tree.clone().build();
                                x.print_nodes();
                                tree.append(new_tree);
                                let x = tree.clone().build();
                                x.print_nodes();
                                println!("\n");
                                break;
                            }
                        }
                    } else {
                        match(state.current_pos) {
                            // Read text into contents
                            Position::InnerContents => node.inner.push(c),
                            // Do nothing (for now)
                            Position::ClosingTag => (),
                            // Do nothing (for now)
                            Position::OpeningTag => (),
                            // Should never happen.
                            Position::Undefined => panic!(),
                            // Should never happen, though a common parse error.
                            Position::UndefinedTag => panic!(),
                            Position::Finished => (),
                        }
                    }
                }
            }
            if let Position::Undefined = state.current_pos{
                return Err(ParseError::UndefinedState);
            }
            state.previous = c;
        }
        // Stop loop if finished.
        if (&mut *input).peekable().peek().is_none() {
            break;
        }
    }
    //println!("Inner : {}", node.inner);
    tree.push_node(node);
        
    return Ok((tree, state));    
}
    
    
fn parse_inner_tag (input : String) {
    println!("Tag : {}", input);
}    


/*
match c {
    '<' => {
        if in_tag {
            //node.inner = inner;
            inner = String::new();
        }
        start_tag = true;
        current = String::new();
    },
    '>' => {
        if start_tag {
            in_tag = true;
            //node.meta = Some(current);
            current = String::new();
        }
        start_tag = false;

        if end_tag {
            //tree.add_child(node);
            //node = Node::new();
            
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
 */