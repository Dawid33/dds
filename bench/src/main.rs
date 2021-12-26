use std::time::Instant;
use log::*;
use simplelog::{CombinedLogger, TermLogger, ColorChoice, TerminalMode, Config};

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        ]
    ).unwrap();

    let html = std::fs::read_to_string("resources/test.html").unwrap();
    let doc = html_parser::preproccesor::PreProccessor::new(&html).unwrap();
    let iter = html_parser::Tokenizer::new(doc);
    
    let start = Instant::now();
    for (_token, _err) in iter {
        //warn!("{:?}", _token);
    }
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
