use preproccesor::PreProccessor;
use std::result::Result;
use states::InsertionMode;
use indextree::Arena;

mod preproccesor;
mod states;
mod tests;
mod tokenizer;
mod parser;

pub use tokenizer::Token;
pub use tokenizer::Tokenizer;

pub trait Tree<T> {
    fn get_node_id(&mut self, node: &T);
}

#[derive(Debug)]
pub enum ElementKind {
    Html,
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
            previous : None
        }
    }
}

// Parse a token stream into a DOM Tree.
// https://html.spec.whatwg.org/multipage/parsing.html
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
                panic!("Tried to reconsume previous token, without a previous token existing.");
            }
        } else if let Some(token) = tokens.next() {
            state.previous = Some(token.clone());
            token
        } else {
            break;
        };

        match state.mode {
            InsertionMode::Initial => parser::parse_initial(current_token, &mut state),
            _ => println!("ERROR: Insertion mode case not handled"),
        }
    }
    Ok(state.tree)
}
