use html_parser::*;
use simplelog::*;
use log::*;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Trace, Config::default(), TerminalMode::Stderr, ColorChoice::Auto),
        ]
    ).unwrap();

    let doc = HtmlParser::parse("<html><head></head><body></body></html>", ParseState::new())?;
    for node in doc.iter() {
        //info!("{:#?}", node);
        debug!("{:?}", node.get());
    }
    Ok(())
}