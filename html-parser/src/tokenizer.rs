use log::*;

use crate::error::HtmlTokenizerError;
use crate::states::*;

use crate::preproccesor::PreProccessor;

pub struct Tokenizer {
    document: PreProccessor,
    state: TokenizationState,
    position: usize,
    previous: Option<char>,
    tag_name_buf: String,
    attributes_buf: Vec<Attribute>,
    is_self_closing: bool,
    tag_kind: TagKind,
    return_state : Option<TokenizationState>,
}

#[derive(Debug, PartialEq)]
pub struct TokenStream {
    pub tokens: Vec<Token>,
}

impl From<Tokenizer> for TokenStream {
    fn from(input: Tokenizer) -> Self {
        let mut output = Vec::new();
        for wrapped_token in input {
            output.push(wrapped_token.unwrap());
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
            return_state : None,
        }
    }
    pub fn reset(&mut self) {
        self.tag_name_buf.clear();
        self.attributes_buf.clear();
        self.is_self_closing = false;
    }
}

impl Iterator for Tokenizer {
    type Item = Result<Token, HtmlTokenizerError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.document.raw[self.position..].chars();
        let mut reconsume = false;

        loop {
            // If the current character is re-consumed, reset the iterator back one character (which is self.position).
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

            if cfg!(feature = "tokenizer-log") {debug!("State : {:?}", self.state);}
            
            match self.state {
                //https://html.spec.whatwg.org/multipage/parsing.html#data-state
                TokenizationState::Data => {
                    match current {
                        '&' => self.state = TokenizationState::CharacterReferenceInData, // &
                        '<' => self.state = TokenizationState::TagOpen,                  // <
                        '\0' => return Some(Err(HtmlTokenizerError::UndefinedError(Token::Character(current)))), // NULL, Parse error
                        _ => return Some(Ok(Token::Character(current))),
                        // TODO : EOF
                    }
                }
                TokenizationState::TagOpen => {
                    match current {
                        '!' => self.state = TokenizationState::MarkupDeclarationOpen, // !
                        '/' => {
                            self.state = TokenizationState::EndTagOpen;
                        } // /
                        'A'..='Z' => {
                            self.state = TokenizationState::TagName;
                            self.tag_name_buf.push(char::to_ascii_lowercase(&current));
                        } // A - Z
                        'a'..='z' => {
                            self.state = TokenizationState::TagName;
                            self.tag_name_buf.push(current);
                        } // a - z
                        '?' => self.state = TokenizationState::BogusComment, // ?, Parse error.
                        _ => return Some(Err(HtmlTokenizerError::UndefinedError(Token::Character('<')))), // Parse error. TODO: Doesn't make sense.
                    }
                }
                TokenizationState::EndTagOpen => {
                    self.tag_kind = TagKind::EndTag;
                    match current {
                        'A'..='Z' => {
                            self.state = TokenizationState::TagName;
                            self.tag_name_buf.push(char::to_ascii_lowercase(&current));
                        } // A - Z
                        'a'..='z' => {
                            self.state = TokenizationState::TagName;
                            self.tag_name_buf.push(current);
                        } // a - z
                        '>' => self.state = TokenizationState::Data, // >, Parse error.
                        _ => self.state = TokenizationState::BogusComment,  // Parse error.
                                                                             //TODO: EOF
                    }
                }
                TokenizationState::TagName => {
                    match current {
                        '\t' | '\u{000A}' | '\u{000C}' | ' ' => {
                            self.state = TokenizationState::BeforeAttributeName
                        } // tab, LF, FF, SPACE
                        '/' => self.state = TokenizationState::SelfClosingStartTag, // /
                        '>' => {
                            self.state = TokenizationState::Data;
                            break;
                        } // >
                        'A'..='Z' => {
                            self.tag_name_buf.push(char::to_ascii_lowercase(&current))
                        } // A - Z
                        '\0' => self.tag_name_buf.push('\u{FFFD}'), // NULL Parse error.
                        _ => self.tag_name_buf.push(current),
                        // TODO : EOF
                    }
                }
                TokenizationState::SelfClosingStartTag => {
                    match current {
                        '>' => {
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
                        '\t' | '\u{000A}' | '\u{000C}' | ' ' => (), // tab, LF, FF, Space
                        '/' => self.state = TokenizationState::SelfClosingStartTag, // /
                        '>' => {
                            self.state = TokenizationState::Data;
                            break;
                        } // >
                        'A'..='Z' => {
                            self.state = TokenizationState::AttributeName;
                            self.attributes_buf.push(Attribute {
                                name: String::from(char::to_ascii_lowercase(&current)),
                                value: String::new(),
                            });
                        } // A - Z
                        '\0' => {
                            self.state = TokenizationState::AttributeName;
                            self.attributes_buf.push(Attribute {
                                name: String::from('\u{FFFD}'),
                                value: String::new(),
                            });
                        } // >
                        '\u{0022}' | '\u{0027}' | '<' | '\u{003D}' => {
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
                        '\t' | '\u{000A}' | '\u{000C}' | ' ' => {
                            self.state = TokenizationState::AfterAttributeName
                        } // tab, LF, FF, Space
                        '/' => self.state = TokenizationState::SelfClosingStartTag, // /
                        '\u{003D}' => self.state = TokenizationState::BeforeAttributeValue, // =
                        '>' => {
                            self.state = TokenizationState::Data;
                            break;
                        } // >
                        'A'..='Z' => {
                            self.attributes_buf
                                .last_mut()
                                .unwrap()
                                .name
                                .push(char::to_ascii_lowercase(&current));
                        } // A - Z
                        '\0' => {
                            self.attributes_buf
                                .last_mut()
                                .unwrap()
                                .name
                                .push('\u{FFFD}');
                        } // NULL, Parse error
                        '\u{0022}' | '\u{0027}' | '<' => {
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
                        '\t' | '\u{000A}' | '\u{000C}' | ' ' => (), // tab, LF, FF, Space
                        '/' => self.state = TokenizationState::SelfClosingStartTag, // /
                        '\u{003D}' => self.state = TokenizationState::BeforeAttributeValue, // =
                        '>' => {
                            self.state = TokenizationState::Data;
                            break;
                        } // >
                        '\0' => {
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
                        '\t' | '\u{000A}' | '\u{000C}' | ' ' => (), // tab, LF, FF, Space
                        '\u{0022}' => self.state = TokenizationState::AttributeValueDoubleQuoted, // "
                        '&' => {
                            self.state = TokenizationState::AttributeValueUnquoted;
                            reconsume = true;
                        } // &
                        '\u{0027}' => self.state = TokenizationState::AttributeValueUnquoted, // '
                        '\0' => {
                            self.state = TokenizationState::AttributeValueUnquoted;
                            self.attributes_buf.last_mut().unwrap().value.push(current);
                        } // NULL, Parse error
                        '>' => {
                            self.state = TokenizationState::Data;
                            break;
                        } // NULL, Parse error
                        '<' | '\u{003D}' | '\u{0060}' => {
                            self.state = TokenizationState::AttributeValueUnquoted;
                            self.attributes_buf.last_mut().unwrap().value.push(current);
                        } // < = `
                        _ => {
                            self.state = TokenizationState::AttributeValueUnquoted;
                            self.attributes_buf.last_mut().unwrap().value.push(current);
                        } // TODO : EOF
                    }
                }
                TokenizationState::AttributeValueDoubleQuoted => {
                    match current {
                        '\"' => self.state = TokenizationState::AfterAttributeValueQuoted,
                        '&' => self.return_state = Some(TokenizationState::AttributeValueDoubleQuoted),
                        '\n' => return Some(Err(HtmlTokenizerError::UnexpectedNullCharacter)),
                        _ => {
                            
                        },
                        //TODO : EOF
                    }
                },
                TokenizationState::AttributeValueSingleQuoted => {

                },
                TokenizationState::AttributeValueUnquoted => {
                    match current {
                        '\t' | '\u{000A}' | '\u{000C}' | ' ' => {
                            self.state = TokenizationState::BeforeAttributeName
                        } // tab, LF, FF, Space
                        '&' => {
                            self.state = TokenizationState::CharacterrReferenceInAttributeValue
                        } // & TODO : What is an allowed additional character in a character reference?
                        '>' => {
                            self.state = TokenizationState::Data;
                            break;
                        }
                        '\0' => {
                            self.attributes_buf
                                .last_mut()
                                .unwrap()
                                .value
                                .push('\u{FFFD}');
                        } // NULL, Parse error
                        '\u{0022}' | '\u{0027}' | '<' | '\u{003D}' | '\u{0060}' => {
                            self.state = TokenizationState::BeforeAttributeName
                        } // " ' < = ` Parse error
                        _ => {
                            self.attributes_buf.last_mut().unwrap().value.push(current);
                        } //TODO : EOF
                    }
                }
                TokenizationState::CharacterReferenceInData => todo!(),
                TokenizationState::RCDATA => todo!(),
                TokenizationState::CharacterReferenceInRCDATA => todo!(),
                TokenizationState::RAWTEXT => todo!(),
                TokenizationState::ScriptData => todo!(),
                TokenizationState::PLAINTEXT => todo!(),
                TokenizationState::RCDATALessThanSign => todo!(),
                TokenizationState::RCDATAEndTagOpen => todo!(),
                TokenizationState::RCDATAEndTagName => todo!(),
                TokenizationState::RAWTEXTLessThanSign => todo!(),
                TokenizationState::RAWTEXTEndTagOpen => todo!(),
                TokenizationState::RAWTEXTEndTagName => todo!(),
                TokenizationState::AttributeValueDoubleQuoted => todo!(),
                TokenizationState::AttributeValueSingleQuoted => todo!(),
                TokenizationState::CharacterrReferenceInAttributeValue => todo!(),
                TokenizationState::AfterAttributeValueQuoted => todo!(),
                TokenizationState::BogusComment => todo!(),
                TokenizationState::MarkupDeclarationOpen => todo!(),
                TokenizationState::CommentStart => todo!(),
                TokenizationState::CommentStartDash => todo!(),
                TokenizationState::Comment => todo!(),
                TokenizationState::CommentEndDash => todo!(),
                TokenizationState::CommentEnd => todo!(),
                TokenizationState::CommentEndBang => todo!(),
                TokenizationState::DOCTYPE => todo!(),
                TokenizationState::BeforeDOCTYPEName => todo!(),
                TokenizationState::DOCTYPEName => todo!(),
                TokenizationState::AfterDOCTYPEName => todo!(),
                TokenizationState::AfterDOCTYPEPublicKeyword => todo!(),
                TokenizationState::BeforeDOCTYPEPublicIdentifier => todo!(),
                TokenizationState::DOCTYPEPublicIdentifierDoubleQuoted => todo!(),
                TokenizationState::DOCTYPEPublicIdentifierSingleQuoted => todo!(),
                TokenizationState::AfterDOCTYPEPublicIdentifier => todo!(),
                TokenizationState::BetweenDOCTYPEPublicAndSystemIdentifiers => todo!(),
                TokenizationState::AfterDOCTYPESystemKeyword => todo!(),
                TokenizationState::BeforeDOCTYPESystemIdentifier => todo!(),
                TokenizationState::DOCTYPESystemIdentifierDoubleQuoted => todo!(),
                TokenizationState::DOCTYPESystemIdentifierSingleQuoted => todo!(),
                TokenizationState::AfterDOCTYPESystemIdentifier => todo!(),
                TokenizationState::BogusDOCTYPE => todo!(),
                TokenizationState::CDATASection => todo!(),
            }
        }

        let output = if let TagKind::StartTag = self.tag_kind {
            Some(Ok(Token::StartTag(self.tag_name_buf.clone(), self.is_self_closing, Vec::from(&mut self.attributes_buf[..]))))
        } else {            
            // The tag kind needs to be reset after every end tag.
            // FIXME: Fix this ^, this is not a good way of doing this.
            self.tag_kind = TagKind::StartTag;
            Some(Ok(Token::EndTag(self.tag_name_buf.clone(), self.is_self_closing, Vec::from(&mut self.attributes_buf[..]))))
        };
        self.reset();
        return output;
    }
}
