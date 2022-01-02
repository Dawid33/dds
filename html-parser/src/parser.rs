use std::{default, string::ParseError};
use indextree::Arena;
use log::*;
use crate::{ParseState, tokenizer::{Token, TagKind}, states::InsertionMode, error::HtmlParseError, preproccesor::PreProccessor, tokenizer::Tokenizer, Element, ElementKind};

//https://html.spec.whatwg.org/multipage/parsing.html#the-initial-insertion-mode
pub fn parse_initial(token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    match &token {
        Token::Character(c) => {
            match c {
                '\t' | '\u{000A}' | '\u{000C}' | ' ' | '\r' => {
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
                '\t' | '\u{000A}' | '\u{000C}' | ' ' | '\r' => {
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
            if cfg!(feature = "parser-log") {trace!("PARSE_BEFORE_HTML {:?}", token);}
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
                '\t' | '\u{000A}' | '\u{000C}' | ' ' | '\r' => {
                    //Do nothing
                }, // tab, LF, FF, Space, Carrige Return
                _ => ()
            }
        },
        Token::Comment(comment) => todo!(),
        Token::DOCTYPE(x,y,z,w) => return Err(HtmlParseError::UnexpectedToken(token)), // Parse error
        Token::StartTag(name, is_self_closing, attributes) => {
            if name == "html" {
                trace!("Shouldn't happen {:?}", token);
                return parse_in_body(token, state);
            } else if name == "head" {
                let new_element = Element{
                    kind : ElementKind::Head,
                };
                let node = state.tree.new_node(new_element);
                state.head_pointer = Some(node);
                state.open_elements.push(node);
                state.mode = InsertionMode::InHead;
                if cfg!(feature = "parser-log") {trace!("PARSE_BEFORE_HEAD {:?}", token);}
            }
        },
        Token::EndTag(name, is_self_closing, attributes) => {
            match name.as_str() {
                "head" | "body" | "html" | "br" => {
                    trace!("PARSE_BEFORE_HTML {:?}", token);
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

// https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inhead
pub fn parse_in_head (token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    match &token {
        Token::EndTag(name,_,_) => {
            if name == "head" {
                if let None = state.open_elements.pop() { 
                    warn!("Tried to pop open_elements stack, but there was nothing to pop.");
                }
                state.mode = InsertionMode::AfterHead;
                if cfg!(feature = "parser-log") {trace!("PARSE_IN_HEAD {:?}", token);}
            }
        }
        _ => {

        }
    }
    Ok(())
}

// https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inheadnoscript
pub fn parse_in_head_noscript (token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    todo!()
}

// https://html.spec.whatwg.org/multipage/parsing.html#the-after-head-insertion-mode
pub fn parse_after_head (token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    match &token {
        Token::StartTag(name,_,_) => {
            state.open_elements.push(state.tree.new_node(Element { kind: ElementKind::Body }));
            state.frame_set_ok = false;
            state.mode = InsertionMode::InBody;
            if cfg!(feature = "parser-log") {trace!("PARSE_AFTER_HEAD {:?}", token);}
        }
        _ => {

        }
    }
    Ok(())
}

// https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inbody
pub fn parse_in_body (token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    match &token {
        Token::EndTag(name, _, _) => {
            if name == "body" {
                state.mode = InsertionMode::AfterBody;
                Ok(())
            } else {
                todo!();
            }
        }
        _ => {
            todo!()
        }
    }
}

// https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-incdata
pub fn parse_text (token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    todo!()
}

// https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-intable
pub fn parse_in_table (token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    todo!()
}

// https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-intabletext
pub fn parse_in_table_text (token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    todo!()
}

// https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-incaption
pub fn parse_in_caption (token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    todo!()
}

// https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-incolgroup
pub fn parse_in_column_group (token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    todo!()
}

// https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-intbody
pub fn parse_in_table_body (token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    todo!()
}

// https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-intr
pub fn parse_in_row (token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    todo!()
}

// https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-intd
pub fn parse_in_cell (token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    todo!()
}

// https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inselect
pub fn parse_in_select (token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    todo!()
}

// https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inselectintable
pub fn parse_in_select_table (token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    todo!()
}

// https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-intemplate
pub fn parse_in_template (token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    todo!()
}

// https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-afterbody
pub fn parse_after_body (token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    match &token {
        Token::EndTag(name, _, _) => {
            if name == "html" {
                state.mode = InsertionMode::AfterAfterBody;
                Ok(())
            } else {
                todo!()
            }
        }
        _ => {
            todo!()
        }
    }
}

// https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inframeset
pub fn parse_in_frameset (token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    todo!()
}

// https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-afterframeset
pub fn parse_after_frameset (token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    todo!()
}

// https://html.spec.whatwg.org/multipage/parsing.html#the-after-after-body-insertion-mode
pub fn parse_after_after_body (token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    todo!()
}

// https://html.spec.whatwg.org/multipage/parsing.html#the-after-after-frameset-insertion-mode
pub fn parse_after_after_frameset (token : Token, state : &mut ParseState) -> Result<(), HtmlParseError> {
    todo!()
}