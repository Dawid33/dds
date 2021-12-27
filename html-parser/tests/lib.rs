// use crate::{HtmlParser, ParseState};

// #[test]
// #[ignore]
// pub fn html_parser_test1() -> Result<(), Box<dyn std::error::Error>>{
//     let wrapped_document = HtmlParser::parse("<html><body></body></html>", ParseState::new());
//     let document = match wrapped_document {
//         Err(e) => {
//             println!("{}", e);
//             return Err(e);
//         },
//         Ok(document) => document,
//     };
    
//     for element in document.iter() {
//         let data = element.get();
//         println!("{:?}", data);
//     }
//     Ok(())
// }
