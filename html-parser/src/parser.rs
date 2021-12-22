use crate::{Token, ParseState, states::InsertionMode, Element, ElementKind};

pub fn parse_initial(token : Token, state : &mut ParseState) {
    match &token {
        Token::Character(c) => {
            match c {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' | '\u{000D}' => {
                    //Do nothing
                }, // tab, LF, FF, Space, Carrige Return
                _ => {
                    state.mode = InsertionMode::BeforeHtml;
                    state.reconsume = true;
                }
            }
        },
        Token::Comment(comment) => todo!(),
        Token::DOCTYPE(x,y,z,w) => todo!(),
        _ => {
            state.mode = InsertionMode::BeforeHtml;
            state.reconsume = true;
        }
    }
}

pub fn parse_before_html(token : Token, state : &mut ParseState) {
    match &token {
        Token::Character(c) => {
            match c {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' | '\u{000D}' => {
                    //Do nothing
                }, // tab, LF, FF, Space, Carrige Return
                _ => ()
            }
        },
        Token::DOCTYPE(x,y,z,w) => (), // Parse error
        Token::Comment(comment) => todo!(),
        _ => {
            let new_element = Element{
                kind : ElementKind::Html,
            };
            state.open_elements.push(state.tree.new_node(new_element));
            state.mode = InsertionMode::BeforeHead;
        }
    }
}