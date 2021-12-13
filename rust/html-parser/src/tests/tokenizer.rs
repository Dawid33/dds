use crate::{document::Document, ValidatedRawDocument, tokenizer::Tokenizer,tokenizer::{Token, TagData}};

#[test]
fn tokenizer_test_1() {
    const TAG1 : &'static str = r#"<html></html>"#;
    let document = ValidatedRawDocument::new(TAG1).unwrap();
    let output = Tokenizer::new(document).collect::<Vec<Token>>();
    let correct_output = vec![
        Token::StartTag(TagData::new().name(String::from("html"))),
        Token::EndTag(TagData::new().name(String::from("html")))
    ];
    assert_eq!(output, correct_output);
}

