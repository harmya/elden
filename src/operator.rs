use crate::utils::extract_operator_and_delimiter;

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
    pub fn new(s: &str) -> (Self, &str) {
        let (operator, rest) = extract_operator_and_delimiter(s.trim());
        let operator = match operator {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            "%" => Self::Mod,
            "!" => Self::Bang,
            "!=" => Self::BangEquals,
            "=" => Self::Equal,
            "==" => Self::EqualEqual,
            ">" => Self::Greater,
            ">=" => Self::GreaterEqual,
            "<" => Self::Less,
            "<=" => Self::LessEqual,
            _ => panic!("Illegal Operator: {}", operator),
        };
        (operator, rest)
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
