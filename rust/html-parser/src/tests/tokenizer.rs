#[allow(unused)]
use crate::{tokenizer::{Token, Tokenizer, TagData}, RawDocument};

#[test]
fn basic_tokenizer_1() {
    const TAG1 : &'static str = r#"<html></html>"#;
    let document = RawDocument::new(TAG1).unwrap();
    let output = Tokenizer::new(document).collect::<Vec<Token>>();
    let correct_output = vec![
        Token::StartTag(TagData::new().name(String::from("html"))),
        Token::EndTag(TagData::new().name(String::from("html")))
    ];
    assert_eq!(output, correct_output);
}

#[test]
fn basic_tokenizer_2() {
    const TAG1 : &'static str = r#"<html><img/></html>"#;
    let document = RawDocument::new(TAG1).unwrap();
    let output = Tokenizer::new(document).collect::<Vec<Token>>();
    let correct_output = vec![
        Token::StartTag(TagData::new().name(String::from("html"))),
        Token::StartTag(TagData::new().name(String::from("img")).self_closing_flag(true)),
        Token::EndTag(TagData::new().name(String::from("html"))),
    ];
    assert_eq!(output, correct_output);
}
