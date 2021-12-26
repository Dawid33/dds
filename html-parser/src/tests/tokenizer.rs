#[allow(unused)]
use crate::tokenizer::{Token, Tokenizer};
#[allow(unused)]
use crate::{preproccesor::PreProccessor, tokenizer::Attribute};

#[test]
fn basic_tokenizer_1() {
    const TAG: &'static str = r#"<html></html>"#;
    let document = PreProccessor::new(TAG).unwrap();
    let output = Tokenizer::new(document).map(|(token, error)| -> Token {token}).collect::<Vec<Token>>();
    let correct_output = vec![
        Token::StartTag("html".to_string(), false, Vec::new()),
        Token::EndTag("html".to_string(), false, Vec::new()),
    ];
    assert_eq!(output, correct_output);
}

#[test]
fn basic_tokenizer_2() {
    const TAG: &'static str = r#"<html><img/></html>"#;
    let document = PreProccessor::new(TAG).unwrap();
    let output = Tokenizer::new(document).map(|(token, error)| -> Token {token}).collect::<Vec<Token>>();
    let correct_output = vec![
        Token::StartTag("html".to_string(), false, Vec::new()),
        Token::StartTag("img".to_string(), true, Vec::new()),
        Token::EndTag("html".to_string(), false, Vec::new()),
    ];
    assert_eq!(output, correct_output);
}

#[test]
fn basic_tokenizer_3() {
    const TAG: &'static str = r#"<html>a<div>b</div>c</html>"#;
    let document = PreProccessor::new(TAG).unwrap();
    let output = Tokenizer::new(document).map(|(token, error)| -> Token {token}).collect::<Vec<Token>>();
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
fn basic_tokenizer_4() {
    const TAG: &'static str = "<html this=100><body nope yes=three>yes</body></html>";
    let document = PreProccessor::new(TAG).unwrap();
    let output = Tokenizer::new(document).map(|(token, error)| -> Token {token}).collect::<Vec<Token>>();
    let correct_output = vec![
        Token::StartTag(
            "html".to_string(),
            false,
            vec![Attribute::new("this", "100")],
        ),
        Token::StartTag(
            "body".to_string(),
            false,
            vec![Attribute::new("nope", ""), Attribute::new("yes", "three")],
        ),
        Token::Character('y'),
        Token::Character('e'),
        Token::Character('s'),
        Token::EndTag("body".to_string(), false, Vec::new()),
        Token::EndTag("html".to_string(), false, Vec::new()),
    ];
    assert_eq!(output, correct_output);
}
