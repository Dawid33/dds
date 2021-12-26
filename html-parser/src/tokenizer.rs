use crate::error::HtmlTokenizerError;
use crate::states::*;

use crate::PreProccessor;

pub struct Tokenizer {
    document: PreProccessor,
    state: TokenizationState,
    position: usize,
    previous: Option<char>,
    tag_name_buf: String,
    attributes_buf: Vec<Attribute>,
    is_self_closing: bool,
    tag_kind: TagKind,
}

#[derive(Debug, PartialEq)]
pub struct TokenStream {
    pub tokens: Vec<Token>,
}

impl From<Tokenizer> for TokenStream {
    fn from(input: Tokenizer) -> Self {
        let mut output = Vec::new();
        for (token, error) in input {
            output.push(token);
        }
        Self { tokens: output }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

impl Attribute {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    DOCTYPE(Option<String>, Option<String>, Option<String>, bool),
    Character(char),
    StartTag(String, bool, Vec<Attribute>),
    EndTag(String, bool, Vec<Attribute>),
    Comment(String),
    EOF,
}

pub enum TagKind {
    StartTag,
    EndTag,
}

impl Tokenizer {
    pub fn new(document: PreProccessor) -> Self {
        Self {
            document,
            position: 0,
            previous: None,
            state: TokenizationState::Data,
            tag_name_buf: String::new(),
            attributes_buf: Vec::new(),
            is_self_closing: false,
            tag_kind: TagKind::StartTag,
        }
    }
    pub fn reset(&mut self) {
        self.tag_name_buf.clear();
        self.attributes_buf.clear();
        self.is_self_closing = false;
    }
}

impl Iterator for Tokenizer {
    type Item = (Token, Option<HtmlTokenizerError>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.document.raw[self.position..].chars();
        let mut reconsume = false;

        loop {
            // If the current character is reconsumed, reset the itertor back one character (which is self.position).
            if reconsume {
                chars =
                    self.document.raw[self.position - self.previous.unwrap().len_utf8()..].chars();
                reconsume = false;
            }

            let result = chars.next();

            let current: char = if let Some(c) = result {
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
                        '\u{003C}' => self.state = TokenizationState::TagOpen,                  // <
                        '\u{0000}' => return Some((Token::Character(current), Some(HtmlTokenizerError::Something))), // NULL, Parse error
                        _ => return Some((Token::Character(current), Some(HtmlTokenizerError::Something))),
                        // TODO : EOF
                    }
                }
                TokenizationState::TagOpen => {
                    match current {
                        '\u{0021}' => self.state = TokenizationState::MarkupDeclarationOpen, // !
                        '\u{002F}' => {
                            self.state = TokenizationState::EndTagOpen;
                        } // /
                        '\u{0041}'..='\u{005A}' => {
                            self.state = TokenizationState::TagName;
                            self.tag_name_buf.push(char::to_ascii_lowercase(&current));
                        } // A - Z
                        '\u{0061}'..='\u{007A}' => {
                            self.state = TokenizationState::TagName;
                            self.tag_name_buf.push(current);
                        } // a - z
                        '\u{003F}' => self.state = TokenizationState::BogusComment, // ?, Parse error.
                        _ => return Some((Token::Character('\u{003C}'), Some(HtmlTokenizerError::Something))), // Parse error. TODO: Doesn't make sense.
                    }
                }
                TokenizationState::EndTagOpen => {
                    self.tag_kind = TagKind::EndTag;
                    match current {
                        '\u{0041}'..='\u{005A}' => {
                            self.state = TokenizationState::TagName;
                            self.tag_name_buf.push(char::to_ascii_lowercase(&current));
                        } // A - Z
                        '\u{0061}'..='\u{007A}' => {
                            self.state = TokenizationState::TagName;
                            self.tag_name_buf.push(current);
                        } // a - z
                        '\u{003E}' => self.state = TokenizationState::Data, // >, Parse error.
                        _ => self.state = TokenizationState::BogusComment,  // Parse error.
                                                                             //TODO: EOF
                    }
                }
                TokenizationState::TagName => {
                    match current {
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                            self.state = TokenizationState::BeforeAttributeName
                        } // tab, LF, FF, SPACE
                        '\u{002F}' => self.state = TokenizationState::SelfClosingStartTag, // /
                        '\u{003E}' => {
                            self.state = TokenizationState::Data;
                            break;
                        } // >
                        '\u{0041}'..='\u{005A}' => {
                            self.tag_name_buf.push(char::to_ascii_lowercase(&current))
                        } // A - Z
                        '\u{0000}' => self.tag_name_buf.push('\u{FFFD}'), // NULL Parse error.
                        _ => self.tag_name_buf.push(current),
                        // TODO : EOF
                    }
                }
                TokenizationState::SelfClosingStartTag => {
                    match current {
                        '\u{003E}' => {
                            self.state = TokenizationState::Data;
                            self.is_self_closing = true;
                            break;
                        } // >
                        _ => {
                            self.state = TokenizationState::BeforeAttributeName;
                            reconsume = true;
                        } // TODO : EOF
                    }
                }
                TokenizationState::BeforeAttributeName => {
                    match current {
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => (), // tab, LF, FF, Space
                        '\u{002F}' => self.state = TokenizationState::SelfClosingStartTag, // /
                        '\u{003E}' => {
                            self.state = TokenizationState::Data;
                            break;
                        } // >
                        '\u{0041}'..='\u{005A}' => {
                            self.state = TokenizationState::AttributeName;
                            self.attributes_buf.push(Attribute {
                                name: String::from(char::to_ascii_lowercase(&current)),
                                value: String::new(),
                            });
                        } // A - Z
                        '\u{0000}' => {
                            self.state = TokenizationState::AttributeName;
                            self.attributes_buf.push(Attribute {
                                name: String::from('\u{FFFD}'),
                                value: String::new(),
                            });
                        } // >
                        '\u{0022}' | '\u{0027}' | '\u{003C}' | '\u{003D}' => {
                            self.state = TokenizationState::AttributeName;
                            self.attributes_buf.push(Attribute {
                                name: String::from(current),
                                value: String::new(),
                            });
                        } // " ' < = Parse error
                        _ => {
                            self.state = TokenizationState::AttributeName;
                            self.attributes_buf.push(Attribute {
                                name: String::from(current),
                                value: String::new(),
                            });
                        } // TODO : EOF
                    }
                }
                TokenizationState::AttributeName => {
                    match current {
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                            self.state = TokenizationState::AfterAttributeName
                        } // tab, LF, FF, Space
                        '\u{002F}' => self.state = TokenizationState::SelfClosingStartTag, // /
                        '\u{003D}' => self.state = TokenizationState::BeforeAttributeValue, // =
                        '\u{003E}' => {
                            self.state = TokenizationState::Data;
                            break;
                        } // >
                        '\u{0041}'..='\u{005A}' => {
                            self.attributes_buf
                                .last_mut()
                                .unwrap()
                                .name
                                .push(char::to_ascii_lowercase(&current));
                        } // A - Z
                        '\u{0000}' => {
                            self.attributes_buf
                                .last_mut()
                                .unwrap()
                                .name
                                .push('\u{FFFD}');
                        } // NULL, Parse error
                        '\u{0022}' | '\u{0027}' | '\u{003C}' => {
                            self.attributes_buf.last_mut().unwrap().name.push(current);
                        } // " ' < = Parse error
                        _ => {
                            self.attributes_buf.last_mut().unwrap().name.push(current);
                        } // TODO : EOF
                    }
                    // TODO: Check if attribute name already exists, and if it does, emit a parse error.
                }
                TokenizationState::AfterAttributeName => {
                    match current {
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => (), // tab, LF, FF, Space
                        '\u{002F}' => self.state = TokenizationState::SelfClosingStartTag, // /
                        '\u{003D}' => self.state = TokenizationState::BeforeAttributeValue, // =
                        '\u{003E}' => {
                            self.state = TokenizationState::Data;
                            break;
                        } // >
                        '\u{0000}' => {
                            self.state = TokenizationState::AttributeName;
                            self.attributes_buf.push(Attribute {
                                name: String::from('\u{FFFD}'),
                                value: String::new(),
                            });
                        } // NULL, Parse error
                        _ => {
                            self.state = TokenizationState::AttributeName;
                            self.attributes_buf.push(Attribute {
                                name: String::from(char::to_ascii_lowercase(&current)),
                                value: String::new(),
                            });
                        } // TODO : EOF
                    }
                }
                TokenizationState::BeforeAttributeValue => {
                    match current {
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => (), // tab, LF, FF, Space
                        '\u{0022}' => self.state = TokenizationState::AttributeValueDoubleQuoted, // "
                        '\u{0026}' => {
                            self.state = TokenizationState::AttributeValueUnquoted;
                            reconsume = true;
                        } // &
                        '\u{0027}' => self.state = TokenizationState::AttributeValueUnquoted, // '
                        '\u{0000}' => {
                            self.state = TokenizationState::AttributeValueUnquoted;
                            self.attributes_buf.last_mut().unwrap().value.push(current);
                        } // NULL, Parse error
                        '\u{003E}' => {
                            self.state = TokenizationState::Data;
                            break;
                        } // NULL, Parse error
                        '\u{003C}' | '\u{003D}' | '\u{0060}' => {
                            self.state = TokenizationState::AttributeValueUnquoted;
                            self.attributes_buf.last_mut().unwrap().value.push(current);
                        } // < = `
                        _ => {
                            self.state = TokenizationState::AttributeValueUnquoted;
                            self.attributes_buf.last_mut().unwrap().value.push(current);
                        } // TODO : EOF
                    }
                }
                TokenizationState::AttributeValueUnquoted => {
                    match current {
                        '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                            self.state = TokenizationState::BeforeAttributeName
                        } // tab, LF, FF, Space
                        '\u{0026}' => {
                            self.state = TokenizationState::CharacterrReferenceInAttributeValue
                        } // & TODO :What is an allowed additional character in a character reference?
                        '\u{003E}' => {
                            self.state = TokenizationState::Data;
                            break;
                        }
                        '\u{0000}' => {
                            self.attributes_buf
                                .last_mut()
                                .unwrap()
                                .value
                                .push('\u{FFFD}');
                        } // NULL, Parse error
                        '\u{0022}' | '\u{0027}' | '\u{003C}' | '\u{003D}' | '\u{0060}' => {
                            self.state = TokenizationState::BeforeAttributeName
                        } // " ' < = ` Parse error
                        _ => {
                            self.attributes_buf.last_mut().unwrap().value.push(current);
                        } //TODO : EOF
                    }
                }
                _ => (),
            }
        }

        let output = if let TagKind::StartTag = self.tag_kind {
            Some((Token::StartTag(
                self.tag_name_buf.clone(),
                self.is_self_closing,
                Vec::from(&mut self.attributes_buf[..]),
            ), None))
        } else {
            Some((Token::EndTag(
                self.tag_name_buf.clone(),
                self.is_self_closing,
                Vec::from(&mut self.attributes_buf[..]),
            ), None))
        };
        self.reset();
        return output;
    }
}
