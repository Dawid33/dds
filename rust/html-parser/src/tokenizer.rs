use crate::states::*;

use crate::RawDocument;

pub struct Tokenizer{
    document : RawDocument,
    state : TokenizationState,
    position : usize,
    previous : Option<char>,
}

#[derive(Debug, PartialEq)]
pub struct TokenStream {
    pub tokens : Vec<Token>,
}

impl From<Tokenizer> for TokenStream {
    fn from(input : Tokenizer) -> Self {
        let mut output = Vec::new();
        for token in input {
            output.push(token);
        }
        Self {
            tokens : output,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Attribute {
    name : String,
    value : String,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    DOCTYPE(Option<String>, Option<String>,Option<String>, bool),
    Character(char),
    StartTag(TagData),
    EndTag(TagData),
    Comment(String),
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct TagData {
    name : String,
    self_closing_flag : bool,
    attributes : Vec<Attribute>
}

impl TagData {
    pub fn new() -> Self {
        Self {
            name : String::new(),
            self_closing_flag : false,
            attributes : Vec::new(),
        }
    }
    pub fn name(mut self, name : String) -> Self {
        self.name = name;
        self
    }
    pub fn self_closing_flag(mut self, flag : bool) -> Self {
        self.self_closing_flag = flag;
        self
    }
    pub fn attributes(mut self, attributes : Vec<Attribute>) -> Self {
        self.attributes = attributes;
        self
    }
}

pub enum TagKind {
    Start,
    End,
}

impl Tokenizer {
    pub fn new (document : RawDocument) -> Self {
        Self {
            document,
            position : 0,
            previous : None,
            state : TokenizationState::Data,
        }
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.document.raw[self.position..].chars();
        let mut tag = TagData::new();
        let mut tag_kind = TagKind::Start;
        let mut reconsumed = false;

        loop {
            // If the current character is reconsumed, reset the itertor.
            if reconsumed {
                chars = self.document.raw[self.position..].chars();
            }

            let result = chars.next();
            
            let current : char = if let Some(c) = result {
                self.previous = Some(c);
                self.position += c.len_utf8();
                c
            } else {
                return None;
            };

            match self.state {
                TokenizationState::Data => {
                    match current {
                        '\u{0026}' => self.state = TokenizationState::CharacterReferenceInData, // &
                        '\u{003C}' => self.state = TokenizationState::TagOpen, // <
                        '\u{0000}' => return Some(Token::Character(current)), // NULL, Parse error
                        _ => return Some(Token::Character(current)),
                        // TODO : EOF
                    }
                },
                TokenizationState::TagOpen => {
                    match current {
                        '\u{0021}' => self.state = TokenizationState::MarkupDeclarationOpen, // !
                        '\u{002F}' => {
                            self.state = TokenizationState::EndTagOpen;
                        }, // /
                        '\u{0041}'..='\u{005A}' => {
                            self.state = TokenizationState::TagName;
                            tag.name.push(char::to_ascii_lowercase(&current));
                        }, // A - Z
                        '\u{0061}'..='\u{007A}' => {
                            self.state = TokenizationState::TagName;
                            tag.name.push(current);
                        }, // a - z
                        '\u{003F}' => self.state = TokenizationState::BogusComment, // ?, Parse error.
                        _ => return Some(Token::Character('\u{003C}')), // Parse error. TODO: Doesn't make sense.
                    }
                },
                TokenizationState::EndTagOpen => {
                    tag_kind = TagKind::End;
                    match current {
                        '\u{0041}'..='\u{005A}' => {
                            self.state = TokenizationState::TagName;
                            tag.name.push(char::to_ascii_lowercase(&current));
                        }, // A - Z
                        '\u{0061}'..='\u{007A}' => {
                            self.state = TokenizationState::TagName;
                            tag.name.push(current);
                        }, // a - z
                        '\u{003E}' => self.state = TokenizationState::Data, // >, Parse error.
                        _ => self.state = TokenizationState::BogusComment, // Parse error.
                        //TODO: EOF
                    }
                },
                TokenizationState::TagName => {
                    match current {
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => self.state = TokenizationState::BeforeAttributeName, // tab, LF, FF, SPACE
                        '\u{002F}' => self.state = TokenizationState::SelfClosingStartTag, // /
                        '\u{003E}' => {
                            self.state = TokenizationState::Data;
                            if let TagKind::Start = tag_kind {
                                return Some(Token::StartTag(tag));
                            } else if let TagKind::End = tag_kind {
                                return Some(Token::EndTag(tag));
                            } else {
                                panic!("Tokenizer should know what tag it is in.");
                            }
                        }, // >
                        '\u{0041}'..='\u{005A}' => tag.name.push(char::to_ascii_lowercase(&current)), // A - Z
                        '\u{0000}' => tag.name.push('\u{FFFD}'), // Parse error.
                        _ => tag.name.push(current),
                        // TODO : EOF
                    }
                },
                TokenizationState::SelfClosingStartTag => {
                    match current {
                        '\u{003E}' => {
                            self.state = TokenizationState::Data;
                            tag.self_closing_flag = true;
                            return Some(Token::StartTag(tag));
                        }, // >
                        _ => {
                            self.position -= current.len_utf8();
                            reconsumed = true;
                            self.state = TokenizationState::BeforeAttributeName;
                        },
                        // TODO : EOF
                    }
                },
                _ => (),
            }

        }
    }
}