use crate::utils::extract_next_digit;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Number(i32),
    Boolean(bool),
}

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);
impl Number {
    pub fn new(s: &str) -> (Self, &str) {
        let (number, rest) = extract_next_digit(s.trim());
        (Self(number.parse().unwrap()), rest)
    }
}

impl Type {
    pub fn new(s: &str) -> (Self, &str) {
        let trimmed = s.trim();
        if let Ok((boolean, rest)) = Self::parse_boolean(trimmed) {
            return (Self::Boolean(boolean), rest);
        }

        if let Ok((number, rest)) = Self::parse_number(trimmed) {
            return (Self::Number(number), rest);
        } else {
            panic!("Invalid input: {}", s);
        }
    }

    pub fn parse_number(s: &str) -> Result<(i32, &str), String> {
        let (number, rest) = extract_next_digit(s.trim());
        if number.is_empty() {
            Err("No valid number found".to_string())
        } else {
            Ok((number.parse().unwrap(), rest))
        }
    }

    pub fn parse_boolean(s: &str) -> Result<(bool, &str), String> {
        if s.starts_with("true") {
            Ok((true, &s[4..]))
        } else if s.starts_with("false") {
            Ok((false, &s[5..]))
        } else {
            Err(format!("Illegal Boolean Value: {}", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::Type;

    #[test]
    fn parse_boolean_true() {
        assert_eq!(Type::parse_boolean("true"), Ok((true, "")));
    }
}
