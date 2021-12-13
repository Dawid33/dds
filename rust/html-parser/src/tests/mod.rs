use crate::tokenizer::TokenStream;

mod tokenizer;

// impl PartialEq for TokenStream {
//     fn eq(&self, other: &Self) -> bool {
//         for (first, second) in (&self.tokens).into_iter().zip(&other.tokens) {
//             if first != second {
//                 return false
//             }
//         }
//         true
//     }
// }