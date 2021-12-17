#![allow(unused)]

mod wasm;
mod exports;
mod arena_tree;

pub fn main() {
    let tree = html_parser::parse::<arena_tree::DefaultTree>("<html></html>").unwrap();
    
}