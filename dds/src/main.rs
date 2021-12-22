#![allow(unused)]

use html_parser::{Element, ParseState, Token};
use indextree::Arena;

//mod wasm; Requires wasmi dependancy
mod arena_tree;
mod exports;

pub fn main() {
    let tree: Arena<Element> =
        html_parser::parse("<html></html>", ParseState::new()).unwrap();
    for node in tree.iter() {
        println!("{:?}", node.get());
    }
}
