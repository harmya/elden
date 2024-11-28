use crate::utils::extract_operator_and_delimiter;

#[derive(Debug, PartialEq)]
pub enum Delimeter {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    SemiColon,
}

impl Delimeter {
    pub fn new(s: &str) -> (Self, &str) {
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
            _ => panic!("Illegal Character: {}", delimeter),
        };
        (delimeter, rest)
    }
}
