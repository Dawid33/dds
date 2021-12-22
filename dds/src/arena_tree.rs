use html_parser::{Element, Token};
use indextree::Arena;

pub struct ArenaTree<Token> {
    arena: indextree::Arena<Token>,
}

impl Default for ArenaTree<Element> {
    fn default() -> Self {
        Self {
            arena: Arena::new(),
        }
    }
}

// impl html_parser::Tree<Element> for ArenaTree<Element> {

// }
