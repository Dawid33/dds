use html_parser::{tokenizer::Tokenizer, preproccesor::PreProccessor};
use simplelog::*;
use log::*;

#[test]
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Trace, Config::default(), TerminalMode::Stderr, ColorChoice::Auto),
        ]
    ).unwrap();

    let doc = PreProccessor::new("<html></html>").unwrap();
    let tokens = Tokenizer::new(doc);
    for token in tokens {
        info!("{:?}", token.unwrap());
    }

    /* Parser Test
    let doc = HtmlParser::parse("<html><head></head><body></body></html>", ParseState::new())?;
    for node in doc.iter() {
        info!("{:#?}", node);
        debug!("{:?}", node.get());
    }
    */

    Ok(())
}