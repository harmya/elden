use crate::{delimeter::Delimeter, keyword::Keyword, operator::Operator, types::Type};

#[derive(Debug, PartialEq)]
pub enum Token {
    Delimter(Delimeter),
    Operator(Operator),
    Literal(Type),
    Keyword(Keyword),
}

impl Token {
    pub fn new(input: &str) -> Result<(Self, &str), String> {
        let remaining = input.trim();

        if let Ok((keyword, rest)) = Keyword::new(remaining) {
            return Ok((Token::Keyword(keyword), rest));
        }

        if let Ok((literal, rest)) = Type::new(remaining) {
            return Ok((Token::Literal(literal), rest));
        }

        if let Ok((operator, rest)) = Operator::new(remaining) {
            return Ok((Token::Operator(operator), rest));
        }

        if let Ok((delimeter, rest)) = Delimeter::new(remaining) {
            return Ok((Token::Delimter(delimeter), rest));
        }

        return Err(format!("Invalid Token {}", remaining));
    }
}
