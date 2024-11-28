use crate::utils::{extract_next_digit, extract_next_ident};

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Number(i32),
    Boolean(bool),
    Identifier(String),
    String(String),
}

impl Type {
    pub fn new(s: &str) -> Result<(Self, &str), String> {
        let trimmed = s.trim();
        if let Ok((boolean, rest)) = Self::parse_boolean(trimmed) {
            return Ok((Self::Boolean(boolean), rest));
        }

        if let Ok((number, rest)) = Self::parse_number(trimmed) {
            return Ok((Self::Number(number), rest));
        }

        if let Ok((string, rest)) = Self::parse_string(trimmed) {
            return Ok((Self::Identifier(string.to_string()), rest));
        } else {
            return Err(format!("Unexpected token: {}", s));
        }
    }

    fn parse_number(s: &str) -> Result<(i32, &str), String> {
        let (number, rest) = extract_next_digit(s.trim());
        if number.is_empty() {
            Err("No valid number found".to_string())
        } else {
            Ok((number.parse().unwrap(), rest))
        }
    }

    fn parse_boolean(s: &str) -> Result<(bool, &str), String> {
        if s.starts_with("true") {
            Ok((true, &s[4..]))
        } else if s.starts_with("false") {
            Ok((false, &s[5..]))
        } else {
            Err(format!("Illegal Boolean Value: {}", s))
        }
    }

    fn parse_string(s: &str) -> Result<(&str, &str), String> {
        let (string, rest) = extract_next_ident(s.trim());
        if string.is_empty() {
            Err(format!("Reached end while parsing"))
        } else {
            Ok((string, rest))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::Type;

    #[test]
    fn parse_boolean_true() {
        assert_eq!(Type::new("true"), Ok((Type::Boolean(true), "")));
    }

    #[test]
    fn parse_number_true() {
        assert_eq!(Type::new("3432"), Ok((Type::Number(3432), "")));
    }
}
