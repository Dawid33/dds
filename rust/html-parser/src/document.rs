use std::result::Result;

use crate::{RawDocument, tree::{Tree, DummyTree}, tokenizer::Tokenizer};

pub struct Document<T> where T : Tree {
    pub tree : T,
}

impl TryFrom<RawDocument> for Document<DummyTree> {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: RawDocument) -> Result<Self, Self::Error> {
        let mut dummy_tree = crate::tree::DummyTree::new();

        let tokens = Tokenizer::new(value);
        for token in tokens {
            dummy_tree.append(token);
        }
        
        let output = Self{
            tree : dummy_tree,
        };
        Ok(output)
    }
}

impl Document<DummyTree> {
    pub fn new(doc : &str) -> Result<Self, Box<dyn std::error::Error>> {
        let raw = RawDocument::new(doc)?;
        let output = Document::try_from(raw)?;
        Ok(output)
    }
}