use crate::tokenizer::Token;

pub trait Tree {
    fn append(&mut self, token : Token);
}

pub struct DummyTree {
}

impl Tree for DummyTree {
    fn append(&mut self, token : Token) {
        //TODO
    }
}

impl DummyTree {
    pub fn new() -> Self {
        Self {}
    }
}