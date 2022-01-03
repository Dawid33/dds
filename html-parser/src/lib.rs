//! A basic HTML Parser
//! 
//! This parser is made up of two main components, the **tokenizer** and the **tree builder**.
//! This parser is specially built for the DDS Project, and as such does not support the entirety of
//! the HTML5 specification. It is however built according to the HTML standard as much as possible.
//! 
//! The main difference is the lack of any scripting capabilities. This allows for better / easier 
//! concurrency as the tree builder will never halt the tokenizer, which allows them to work in parallel.
//! Since scripting capabilities are not needed for the DDS project, it makes sense to just leave them out and
//! to instead optimize everything for the parser's intended use case.
//! 
//! ## Parsing steps.
//! 1. Pre-Processor : This takes the raw HTML input and does some basic checks to see whether it can be parsed by the tokenizer.
//! 2. Tokenizer : Parses the raw data into tokens that are semantically useful.
//! 3. Tree Builder : Takes a stream of tokens and constructs a DOM Tree out of it.
//! 
//! Goals : 
//! - [ ] Remove all dependencies besides log. This will be achieved once I write a trait abstraction over a Tree structure. That way the parser could construct a HTML DOM using any data structure that implements this trait. This would be useful for having immutable / mutable / concurrent tree implementations without writing parsing code that individually generates these structures.
//! - [ ] SIMD / GPU Acceleration : I hope this is possible, though I'm not sure. Research [simd_json](https://simdjson.org/) and see how they managed it. Some ideas on how to do this :
//!     - Copy the entire file into the GPU and concurrently check every byte to see what range of characters it falls into and how it affects the state. This would create an array of bytes equal in size to the original file. This might allow the tokenizer to "predict" what needs to be done next.
//! - [ ] 
#![allow(unused)]

pub mod error;
pub mod parser;
pub mod preproccesor;
pub mod states;
pub mod tokenizer;
mod tests;

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
            // Here we need to get the next token we need to process.
            // Check if we need to re-consume a previously processed token.
            let current_token = if state.reconsume {
                if let Some(ref token) = state.previous {
                    if cfg!(feature = "parser-log") {info!("Reconsumed token {:?}", token);}
                    state.reconsume = false;
                    token.clone()
                } else {
                    // Cannot re-consume a previously processed token if no token has been processed.
                    return Err(Box::new(HtmlParseError::ReconsumeNonExistingToken));
                }
            // Get the next token from the tokenizer.
            } else if let Some(wrapped_token) = tokens.next() {
                let token = match wrapped_token {
                    Ok(token) => token,
                    Err(e) => {
                        match e {
                            error::HtmlTokenizerError::UndefinedError(token) => token,
                            error::HtmlTokenizerError::UnexpectedNullCharacter => return Err(Box::new(e)),
                        }
                    }
                };
                if cfg!(feature = "parser-log") {info!("Token : {:?}", token);}
                // NOTE: Previously handled tokens are only needed when re-consume a previous token. This could be optimised so that the previous token is only set when necessary (aka if the token needs to be re-consumed later on).
                state.previous = Some(token.clone());
                token
            } else {
                // Exit the loop if tokens.next() returns None, which means we have read all tokens. This is the equivalent of hitting EOF.
                break;
            };

            // These functions should (hopefully) be inlined by the compiler.
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
                return Err(Box::new(e));
            }
        }

        // Return a fully constructed tree.
        Ok(state.tree)
    }
}