// use html_parser::Tokenizer;
// use html_parser::Token;
use argparse::{ArgumentParser, Store};

struct Options {
    file: String,
}

impl Options {
    pub fn new() -> Self {
        Self {
            file: String::new(),
        }
    }
}

fn main() {
    let mut options = Options::new();
    {
        let mut parser = ArgumentParser::new();
        parser
            .refer(&mut options.file)
            .add_option(&["-f", "--file"], Store, "Command to run");
        parser.set_description("This program takes html from stdin and outputs it prettified. It continues to output until an EOF character is hit, or until it encounters a parse error.");
        parser.parse_args_or_exit();
    }
}
