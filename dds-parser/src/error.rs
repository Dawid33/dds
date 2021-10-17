use std::fmt::Display;

#[derive(Debug)]
pub enum ParseError {
    NodeNotExist
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error!")
    }
}
