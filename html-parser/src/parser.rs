use std::{default, string::ParseError};
use indextree::Arena;
use log::*;
use crate::{Token, states::InsertionMode, error::HtmlParseError, preproccesor::PreProccessor, Tokenizer};


pub trait Tree<T> {
    fn get_node_id(&mut self, node: &T);
}

#[derive(Debug)]
pub enum ElementKind {
    Html,
    Head,
    Body,
    Br,
}

#[derive(Debug)]
pub struct Element {
    kind : ElementKind,
}

pub struct ParseState {
    _script_nesting_level: u32,
    _parser_pause: bool,
    tree: Arena<Element>,
    mode: InsertionMode,
    open_elements: Vec<indextree::NodeId>,
    reconsume : bool,
    previous : Option<Token>,
    head_pointer : Option<indextree::NodeId>,
}

impl ParseState {
    pub fn new() -> Self {
        Self {
            _script_nesting_level: 0,
            _parser_pause: false,
            mode: InsertionMode::Initial,
            open_elements: Vec::new(),
            tree : Arena::new(),
            reconsume : false,
            previous : None,
            head_pointer : None,
        }
    }
}

pub struct HtmlParser {}

impl HtmlParser {
    pub fn new() -> Self {
        Self {}
    }

     // Parse a token stream into a DOM Tree.
    // https://html.spec.whatwg.org/multipage/parsing.html#tree-construction
    pub fn parse(input: &str, mut state: ParseState) -> Result<Arena<Element>, Box<dyn std::error::Error>>
    //where T : Tree<Element> + Default
    {
        let html = PreProccessor::new(input)?;
        let mut tokens = Tokenizer::new(html);
        
        // Exit the loop after all tokens have been parsed.
        loop {
            let current_token = if state.reconsume {
                if let Some(ref token) = state.previous {
                    state.reconsume = false;
                    token.clone()
                } else {
                    return Err(Box::new(HtmlParseError::ReconsumeNonExistingToken));
                }
            } else if let Some((token, error)) = tokens.next() {
                if let Some(e) = error {
                    warn!("{}", e);
                }
                info!("Token : {:?}", token);
                state.previous = Some(token.clone());
                token
            } else {
                // Exit the loop if tokens.next() return None, which means we have read all tokens.
                break;
            };

            let result = match state.mode {
                InsertionMode::Initial => parse_initial(current_token, &mut state),
                InsertionMode::BeforeHtml => parse_before_html(current_token, &mut state),
                InsertionMode::BeforeHead => parse_before_head(current_token, &mut state),
                _ => return Err(Box::new(HtmlParseError::InsertionModeCaseNotHandled(state.mode))),
            };
            
            if let Err(e) = result {

            }
        }
        Ok(state.tree)
    }
}

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