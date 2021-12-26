use std::{default, string::ParseError};

use crate::{Token, ParseState, states::InsertionMode, Element, ElementKind, error::HtmlParseError};

//https://html.spec.whatwg.org/multipage/parsing.html#the-initial-insertion-mode
pub fn parse_initial(token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
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
    Ok(())
}

//https://html.spec.whatwg.org/multipage/parsing.html#the-before-html-insertion-mode
pub fn parse_before_html(token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    match &token {
        Token::DOCTYPE(x,y,z,w) => return Err(HtmlParseError::UnexpectedToken(token)), // Parse error
        Token::Comment(comment) => todo!(),
        Token::Character(c) => {
            match c {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' | '\u{000D}' => {
                    //Do nothing
                }, // tab, LF, FF, Space, Carrige Return
                _ => ()
            }
        },
        token => {
            let new_element = Element{
                kind : ElementKind::Html,
            };
            state.open_elements.push(state.tree.new_node(new_element));
            state.mode = InsertionMode::BeforeHead;
            if let Token::StartTag(name, _, _) = token {
                if name != "html" {
                    state.reconsume = true;
                }
            }
        }
    }
    Ok(())
}

// https://html.spec.whatwg.org/multipage/parsing.html#the-before-head-insertion-mode
pub fn parse_before_head (token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    #[inline]
    fn default(token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
        let new_element = Element{
            kind : ElementKind::Head,
        };
        state.open_elements.push(state.tree.new_node(new_element));
        Ok(())
    }
    match &token {
        Token::Character(c) => {
            match c {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' | '\u{000D}' => {
                    //Do nothing
                }, // tab, LF, FF, Space, Carrige Return
                _ => ()
            }
        },
        Token::Comment(comment) => todo!(),
        Token::DOCTYPE(x,y,z,w) => return Err(HtmlParseError::UnexpectedToken(token)), // Parse error
        Token::StartTag(name, is_self_closing, attributes) => {
            if name == "html" {
                return parse_in_body(token, state);
            } else if name == "head" {
                let new_element = Element{
                    kind : ElementKind::Head,
                };
                let node = state.tree.new_node(new_element);
                state.head_pointer = Some(node);
                state.open_elements.push(node);
                state.mode = InsertionMode::InHead;
            }
        },
        Token::EndTag(name, is_self_closing, attributes) => {
            match name.as_str() {
                "head" | "body" | "html" | "br" => {
                    return default(token, state);
                }
                _ => {
                    return Err(HtmlParseError::GenericParseError);
                }
            }
        },
        _ => {
            return default(token, state);
        },
    }
    Ok(())
}

// https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inbody
pub fn parse_in_body (token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    Ok(())
}