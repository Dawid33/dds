use std::{fmt::Display, error::Error};

use crate::{Tokenizer, Token, states::InsertionMode};

#[derive(Debug)]
pub enum HtmlParseError {
    InsertionModeCaseNotHandled(InsertionMode),
    ReconsumeNonExistingToken,
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
        }
    }
}

#[derive(Debug)]
pub enum HtmlTokenizerError {
    Something,
}

impl std::error::Error for HtmlTokenizerError {}

impl std::fmt::Display for HtmlTokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HtmlTokenizerError: ");
        match &self {
            HtmlTokenizerError::Something => write!(f, "Something went wrong."),
        }
    }
}