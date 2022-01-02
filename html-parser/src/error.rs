use std::{fmt::Display, error::Error};

use crate::{tokenizer::Tokenizer, tokenizer::Token, states::InsertionMode};

#[derive(Debug)]
pub enum HtmlParseError {
    InsertionModeCaseNotHandled(InsertionMode),
    ReconsumeNonExistingToken,
    UnexpectedToken(Token),
    GenericParseError,
}


impl std::error::Error for HtmlParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

impl std::fmt::Display for HtmlParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HtmlParseError: ");
        match &self {
            HtmlParseError::InsertionModeCaseNotHandled(mode) => write!(f, "InsertionModeCaseNotHandled. Missing implementation of {:?}", mode),
            HtmlParseError::ReconsumeNonExistingToken => write!(f, "ReconsumeNonExistingToken. Fatal implementation error."),
            HtmlParseError::UnexpectedToken(t) => write!(f, "UnexpectedToken {:?}", t),
            HtmlParseError::GenericParseError => write!(f, "GenericParseError. Explicit cause is not documented."),
        }
    }
}

#[derive(Debug)]
pub enum HtmlTokenizerError {
    UndefinedError(Token),
    UnexpectedNullCharacter,
}

impl std::error::Error for HtmlTokenizerError {}

impl std::fmt::Display for HtmlTokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HtmlTokenizerError: ");
        match &self {
            HtmlTokenizerError::UndefinedError(token) => write!(f, "Encountered an undefined error when tokenizing. Last token = {:?}.", token),
            HtmlTokenizerError::UnexpectedNullCharacter => write!(f, "Encountered a NULL character when there wasn't supposed to be any."),
        }
    }
}