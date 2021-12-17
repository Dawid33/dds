use std::{result::Result};
use preproccesor::PreProccessor;

mod tests;
mod tokenizer;
mod preproccesor;
mod states;

pub use tokenizer::Token;
pub use tokenizer::Tokenizer;

pub trait Tree {
    fn append_child(&mut self, token : Token);
    fn append_sibling(&mut self, token : Token);
}

pub struct ParseState{
    script_nesting_level : u32,
    parser_pause : bool,
}

pub fn parse<T>(input : &str) -> Result<T, Box<dyn std::error::Error>>
where T : Tree + Default {
    let mut tree= T::default();
    let html = PreProccessor::new(input)?;
    let tokens = Tokenizer::new(html);
    for token in tokens {
        tree.append_sibling(token);
    }
    Ok(tree)
}
