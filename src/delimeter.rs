use crate::utils::{extract_next_literal, extract_operator_and_delimiter};

#[derive(Debug, PartialEq)]
pub enum Delimeter {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    SemiColon,
    DoubleQuote,
}

impl Delimeter {
    pub fn new(s: &str) -> Result<(Self, &str), String> {
        let (delimeter, rest) = match extract_operator_and_delimiter(s.trim()) {
            Ok((delimeter, rest)) => (delimeter, rest),
            Err(e) => panic!("{}", e),
        };
        let delimeter = match delimeter {
            "(" => Self::LeftParen,
            ")" => Self::RightParen,
            "{" => Self::LeftBrace,
            "}" => Self::RightBrace,
            ";" => Self::SemiColon,
            "\"" => Self::DoubleQuote,
            _ => return Err(format!("Illegal Character: {}", delimeter)),
        };
        Ok((delimeter, rest))
    }

    pub fn process_literal(s: &str) -> Result<(&str, &str), String> {
        let (literal, rest) = extract_next_literal(s.trim());
        match rest.chars().next() {
            Some(ch) => {
                if ch == '"' {
                    return Ok((literal, &rest[1..]));
                } else {
                    return Err(format!("Expected \""));
                }
            }
            None => return Err(format!("Reached end while parsing \"")),
        };
    }
}
