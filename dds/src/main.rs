extern crate simplelog;

use simplelog::*;
use html_parser::{HtmlParser, ParseState};
use log::*;

fn main () -> Result<(), Box<dyn std::error::Error>> {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        ]
    ).unwrap();

    let wrapped_document = HtmlParser::parse("<html></html>", ParseState::new());
    let document = match wrapped_document {
        Err(e) => {
            error!("{:?}", e.to_string());
            return Err(e);
        },
        Ok(document) => document,
    };

    for element in document.iter() {
        let data = element.get();
        println!("{:?}", data);
    }
    Ok(())
}