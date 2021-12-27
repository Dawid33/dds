#![allow(unused)]
use preproccesor::PreProccessor;
use simplelog::ColorChoice;
use simplelog::CombinedLogger;
use simplelog::Config;
use simplelog::TermLogger;
use simplelog::TerminalMode;
use std::fs;
use std::result::Result;
use states::InsertionMode;
use indextree::Arena;
use log::*;
use pico_args::Arguments;

pub mod preproccesor;
pub mod states;
pub mod tests;
pub mod tokenizer;
pub mod parser;
pub mod error;

use error::HtmlParseError;
pub use tokenizer::Token;
pub use tokenizer::Tokenizer;

const HELP: &str = "\
App

USAGE:
  app [OPTIONS] --number NUMBER [INPUT]

FLAGS:
  -h, --help            Prints help information

OPTIONS:
  --number NUMBER       Sets a number
  --opt-number NUMBER   Sets an optional number
  --width WIDTH         Sets width [default: 10]
  --output PATH         Sets an output path

ARGS:
  <INPUT>
";

#[derive(Debug)]
struct Args {
    input: String,
}

impl Default for Args {
    fn default() -> Self {
        Self { input: Default::default() }
    }
}

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Trace, Config::default(), TerminalMode::Stderr, ColorChoice::Auto),
        ]
    ).unwrap();

    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            match e {
                pico_args::Error::NonUtf8Argument => todo!(),
                pico_args::Error::MissingArgument => todo!(),
                pico_args::Error::MissingOption(_) => todo!(),
                pico_args::Error::OptionWithoutAValue(_) => todo!(),
                pico_args::Error::Utf8ArgumentParsingFailed { value, cause } => todo!(),
                pico_args::Error::ArgumentParsingFailed { cause } => todo!(),
            }
            error!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let html_file = std::fs::read_to_string(args.input);

    let wrapped_document = parser::HtmlParser::parse("<html></html>", parser::ParseState::new());
    let document = match wrapped_document {
        Ok(document) => document,
        Err(e) => {
            error!("{:?}", e.to_string());
            std::process::exit(1);
        },
    };

    
}

fn parse_args() -> Result<Args, pico_args::Error> {
    let mut pico_args_output = pico_args::Arguments::from_env();
    
    // Help has a higher priority and should be handled separately.
    if pico_args_output.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let args = Args {
        input: pico_args_output.free_from_str()?,
    };

    // It's up to the caller what to do with the remaining arguments.
    let remaining = pico_args_output.finish();
    if !remaining.is_empty() {
        warn!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}
