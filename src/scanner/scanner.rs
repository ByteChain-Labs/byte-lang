use std::collections::HashMap;

use crate::byte::Byte;
use crate::token::Token;
use crate::token_type::TokenType;

// fn keywords() -> HashMap<&'static str, TokenType> {
//     let mut keywords = HashMap::new();
//     keywords.insert("and", TokenType::AND);
//     keywords.insert("class", TokenType::CLASS);
//     keywords.insert("contract", TokenType::CONTRACT);
//     keywords.insert("else", TokenType::ELSE);
//     keywords.insert("false", TokenType::FALSE);
//     keywords.insert("for", TokenType::FOR);
//     keywords.insert("func", TokenType::FUNC);
//     keywords.insert("if", TokenType::IF);
//     keywords.insert("nil", TokenType::NIL);
//     keywords.insert("or", TokenType::OR);
//     keywords.insert("print", TokenType::PRINT);
//     keywords.insert("return", TokenType::RETURN);
//     keywords.insert("super", TokenType::SUPER);
//     keywords.insert("self", TokenType::SELF);
//     keywords.insert("true", TokenType::TRUE);
//     keywords.insert("let", TokenType::LET);
//     keywords.insert("const", TokenType::CONST);
//     keywords.insert("while", TokenType::WHILE);
// }
