#[allow(unused)]
use crate::tokenizer::{Token, Tokenizer};
#[allow(unused)]
use crate::{preproccesor::PreProccessor, tokenizer::Attribute};

#[test]
fn tokenizer_basic() {
    const TAG: &'static str = r#"<html></html>"#;
    let document = PreProccessor::new(TAG).unwrap();
    let output = Tokenizer::new(document)
        .map(|wrapped_token| -> Token {wrapped_token.unwrap()}).collect::<Vec<Token>>();
    let correct_output = vec![
        Token::StartTag("html".to_string(), false, Vec::new()),
        Token::EndTag("html".to_string(), false, Vec::new()),
    ];
    assert_eq!(output, correct_output);
}

#[test]
fn tokenizer_multiple_tags() {
    const TAG: &'static str = r#"<html>a<div>b</div>c</html>"#;
    let document = PreProccessor::new(TAG).unwrap();
    let output = Tokenizer::new(document)
        .map(|wrapped_token| -> Token {wrapped_token.unwrap()}).collect::<Vec<Token>>();
    let correct_output = vec![
        Token::StartTag("html".to_string(), false, Vec::new()),
        Token::Character('a'),
        Token::StartTag("div".to_string(), false, Vec::new()),
        Token::Character('b'),
        Token::EndTag("div".to_string(), false, Vec::new()),
        Token::Character('c'),
        Token::EndTag("html".to_string(), false, Vec::new()),
    ];
    assert_eq!(output, correct_output);
}

#[test]
fn tokenizer_self_closing_tag() {
    const TAG: &'static str = r#"<html><img/></html>"#;
    let document = PreProccessor::new(TAG).unwrap();
    let output = Tokenizer::new(document)
        .map(|wrapped_token| -> Token {wrapped_token.unwrap()}).collect::<Vec<Token>>();
    let correct_output = vec![
        Token::StartTag("html".to_string(), false, Vec::new()),
        Token::StartTag("img".to_string(), true, Vec::new()),
        Token::EndTag("html".to_string(), false, Vec::new()),
    ];
    assert_eq!(output, correct_output);
}



#[test]
fn tokenizer_tag_attribute() {
    const TAG: &'static str = "<html this=100 other ohYeah=yes ></html>";
    let document = PreProccessor::new(TAG).unwrap();
    let output = Tokenizer::new(document)
        .map(|wrapped_token| -> Token {wrapped_token.unwrap()}).collect::<Vec<Token>>();
    let correct_output = vec![
        Token::StartTag(
            "html".to_string(),
            false,
            vec![
                Attribute::new("this", "100"),
                Attribute::new("other", ""),
                // Capital letters in attributes keys are turned to lower case.
                Attribute::new("ohyeah", "yes")
                ],
        ),
        Token::EndTag("html".to_string(), false, Vec::new()),
    ];
    assert_eq!(output, correct_output);
}

