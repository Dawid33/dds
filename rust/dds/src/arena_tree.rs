use html_parser::Token;

pub struct DefaultTree {

}

impl Default for DefaultTree {
    fn default() -> Self {
        Self {  }
    }
} 

impl html_parser::Tree for  DefaultTree {
    fn append_child(&mut self, token : Token) {
        todo!()
    }

    fn append_sibling(&mut self, token : Token) {
        todo!()
    }
}