use std::string::ParseError;

use crate::token::Token;

pub fn is_operator(token: Token) -> Result<bool, ParseError> {
    match token {
        Token::Add
        | Token::Sub
        | Token::Mul
        | Token::Div
        | Token::Mod
        | Token::NotEqual
        | Token::EqualEqual
        | Token::Greater
        | Token::GreaterEqual
        | Token::Less
        | Token::LessEqual
        | Token::Equal
        | Token::Not
        | Token::Or
        | Token::And => Ok(true),
        _ => Ok(false),
    }
}
