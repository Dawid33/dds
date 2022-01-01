#![allow(unused)]

mod error;
mod parser;
mod preproccesor;
mod states;
mod tokenizer;

use std::{default, string::ParseError};
use indextree::Arena;
use log::*;
use crate::{tokenizer::Token, states::InsertionMode, error::HtmlParseError, preproccesor::PreProccessor, tokenizer::Tokenizer};
use parser::*;


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
    frame_set_ok : bool,
    reconsume : bool,
    tree: Arena<Element>,
    mode: InsertionMode,
    open_elements: Vec<indextree::NodeId>,
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
            frame_set_ok : true,
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
                    if cfg!(feature = "parser-log") {info!("Reconsumed token {:?}", token);}
                    state.reconsume = false;
                    token.clone()
                } else {
                    return Err(Box::new(HtmlParseError::ReconsumeNonExistingToken));
                }
            } else if let Some((token, error)) = tokens.next() {
                if let Some(e) = error {
                    warn!("{}", e);
                }
                if cfg!(feature = "parser-log") {info!("Token : {:?}", token);}
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
                InsertionMode::InHead => parse_in_head(current_token, &mut state),
                InsertionMode::InHeadNoScript => parse_in_head_noscript(current_token, &mut state),
                InsertionMode::AfterHead => parse_after_head(current_token, &mut state),
                InsertionMode::InBody => parse_in_body(current_token, &mut state),
                InsertionMode::Text => parse_text(current_token, &mut state),
                InsertionMode::InTable => parse_in_table(current_token, &mut state),
                InsertionMode::InTableText => parse_in_table_text(current_token, &mut state),
                InsertionMode::InCaption => parse_in_caption(current_token, &mut state),
                InsertionMode::InColumnGroup => parse_in_column_group(current_token, &mut state),
                InsertionMode::InTableBody => parse_in_table_body(current_token, &mut state),
                InsertionMode::InRow => parse_in_row(current_token, &mut state),
                InsertionMode::InCell => parse_in_cell(current_token, &mut state),
                InsertionMode::InSelect => parse_in_select(current_token, &mut state),
                InsertionMode::InSelectTable => parse_in_select_table(current_token, &mut state),
                InsertionMode::AfterBody => parse_after_body(current_token, &mut state),
                InsertionMode::InFrameset => parse_in_frameset(current_token, &mut state),
                InsertionMode::AfterFrameset => parse_after_frameset(current_token, &mut state),
                InsertionMode::AfterAfterBody => parse_after_body(current_token, &mut state),
                InsertionMode::AfterAfterFrameset => parse_after_after_frameset(current_token, &mut state),
            };
            
            if let Err(e) = result {

            }
        }
        Ok(state.tree)
    }
}