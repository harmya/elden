use crate::utils::{self, extract_operator_and_delimiter, tag};

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Bang,
    BangEquals,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl Operator {
    pub fn new(s: &str) -> Result<(Self, &str), String> {
        tag("+", s)
            .map(|s| (Self::Add, s))
            .or_else(|_| tag("-", s).map(|s| (Self::Sub, s)))
            .or_else(|_| tag("*", s).map(|s| (Self::Mul, s)))
            .or_else(|_| tag("/", s).map(|s| (Self::Div, s)))
            .or_else(|_| tag("%", s).map(|s| (Self::Mod, s)))
            .or_else(|_| tag("!", s).map(|s| (Self::Bang, s)))
            .or_else(|_| tag("!=", s).map(|s| (Self::BangEquals, s)))
            .or_else(|_| tag("==", s).map(|s| (Self::EqualEqual, s)))
            .or_else(|_| tag("=", s).map(|s| (Self::Equal, s)))
            .or_else(|_| tag(">", s).map(|s| (Self::Greater, s)))
            .or_else(|_| tag(">=", s).map(|s| (Self::GreaterEqual, s)))
            .or_else(|_| tag("<", s).map(|s| (Self::Less, s)))
            .or_else(|_| tag("<=", s).map(|s| (Self::LessEqual, s)))
    }
}

#[derive(Debug, PartialEq)]
pub enum Delimeter {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
}

impl Delimeter {
    pub fn new(s: &str) -> (Self, &str) {
        let (delimeter, rest) = extract_operator_and_delimiter(s.trim());
        let delimeter = match delimeter {
            "(" => Self::LeftParen,
            ")" => Self::RightParen,
            "{" => Self::LeftBrace,
            "}" => Self::RightBrace,
            "," => Self::Comma,
            "." => Self::Dot,
            _ => panic!("Illegal Character: {}", delimeter),
        };
        (delimeter, rest)
    }
}
