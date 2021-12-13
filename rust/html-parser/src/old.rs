#![allow(unused)]
mod tests;
mod error;
mod element;

use std::{borrow::{Borrow, BorrowMut}, cell::RefCell, ops::Deref, rc::Rc, str::Chars};

use error::*;
use indextree::{Arena, NodeId};
use element::{Element, ElementKind};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Position {
    OpeningElement,
    ClosingElement,
    InnerContents,
    UndefinedElement,
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

pub fn parse_tag<'a,'b>(mut input : &'a mut Chars<'b>, mut state : ParseState, arena : &mut Arena<Element>, root : NodeId) -> Result<(),ParseError> {
    let mut current_element = Element::new();
    let mut current_root : Option<NodeId> = Some(root);
    let mut current_child : Option<NodeId> = None;
    let mut should_go_deeper = false;

    while (state.current_pos != Position::Finished) {
        for c in &mut *input {
            match c {
                //Entering either opening tag or closing tag.
                '<' => {
                    state.current_pos = match(state.current_pos) {
                        Position::InnerContents => Position::UndefinedElement,
                        // '<' cannot exist if still inside a closing tag, error.
                        Position::ClosingElement => panic!(),
                        // Same error as if we are inside a closing tag.
                        Position::OpeningElement => panic!(),
                        // Should never happen.
                        Position::UndefinedElement => panic!(),
                        // Should never happen.
                        Position::Undefined => panic!(),
                        // Should never happen.
                        Position::Finished => Position::Finished,
                    }
                }
                //Leaving either opening tag or closing tag.
                '>' => {
                    state.current_pos = match(state.current_pos) {
                        // Moving inside element.
                        Position::OpeningElement => Position::InnerContents,
                        // Leaving tag, should return element.
                        Position::ClosingElement => {
                            state.current_pos = Position::Finished;
                            break
                        }
                        // Cannot be inside inner contents.
                        Position::InnerContents => panic!(),
                        Position::Undefined => panic!(),
                        Position::UndefinedElement => panic!(),
                        Position::Finished => Position::Finished,
                    }
                },
                _ => {
                    if state.previous == '<' && state.current_pos == Position::UndefinedElement {
                        match(c) {
                            '/' => {
                                state.current_pos = Position::ClosingElement;
                            },
                            _ => {
                                should_go_deeper = true;
                                state.current_pos = Position::InnerContents;
                                break;
                            }
                        }
                    } else {
                        match(state.current_pos) {
                            // Read text into contents
                            Position::InnerContents => current_element.inner.push(c),
                            // Do nothing (for now)
                            Position::ClosingElement => (),
                            // Do nothing (for now)
                            Position::OpeningElement => (),
                            // Should never happen.
                            Position::Undefined => panic!(),
                            // Should never happen, though a common parse error.
                            Position::UndefinedElement => panic!(),
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
        // Restarts function for child element.
        if should_go_deeper {
            println!("Element: {}", current_element.inner);
            let mut new_state = state.clone();
            new_state.current_pos = Position::OpeningElement;
            
            let child = arena.new_node(Element::new());
            parse_tag(input, new_state, arena, child)?;
            // Loop through them and add them to the current node.
            root.append(child, arena);

            if let Some(current) = current_child {
                current.insert_after(child, arena);
            }

            current_child = Some(child);

            should_go_deeper = false;
        }
        // Stop loop if finished.
        if input.peekable().peek().is_none() {
            break;
        }
    }

    let n = arena.new_node(current_element);
    if let Some(child) = current_child {
        child.insert_after(n, arena);
    }

    return Ok(());
}
    
fn parse_inner_tag (input : String) {
    println!("Element : {}", input);
}