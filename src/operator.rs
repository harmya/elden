use crate::utils::extract_operator_and_delimiter;

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BangEquals,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    Not,
    Or,
    And,
}

impl Operator {
    pub fn new(s: &str) -> Result<(Self, &str), String> {
        let (operator, rest) = match extract_operator_and_delimiter(s.trim()) {
            Ok((operator, rest)) => (operator, rest),
            Err(err) => panic!("{}", err),
        };

        let op = match operator {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            "%" => Self::Mod,
            "!=" => Self::BangEquals,
            "==" => Self::EqualEqual,
            ">" => Self::Greater,
            ">=" => Self::GreaterEqual,
            "<" => Self::Less,
            "<=" => Self::LessEqual,
            "=" => Self::Equal,
            "!" => Self::Not,
            "||" => Self::Or,
            "&&" => Self::And,
            _ => return Err(format!("Illegal Operator: {}", operator)),
        };
        Ok((op, rest))
    }

    pub fn tag(s: &str) -> bool {
        matches!(
            s,
            "+" | "-"
                | "*"
                | "/"
                | "%"
                | "!="
                | "=="
                | ">"
                | ">="
                | "<"
                | "<="
                | "="
                | "!"
                | "||"
                | "&&"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_add_with_whitespace() {
        assert_eq!(Operator::new("  +  "), Ok((Operator::Add, "")));
    }

    #[test]
    fn parse_subtract_with_whitespace() {
        assert_eq!(Operator::new("  -  "), Ok((Operator::Sub, "")));
    }

    #[test]
    fn parse_multiply_with_whitespace() {
        assert_eq!(Operator::new("  *  "), Ok((Operator::Mul, "")));
    }

    #[test]
    fn parse_divide_with_whitespace() {
        assert_eq!(Operator::new("  /  "), Ok((Operator::Div, "")));
    }

    #[test]
    fn parse_modulus_with_whitespace() {
        assert_eq!(Operator::new("  %  "), Ok((Operator::Mod, "")));
    }

    #[test]
    fn parse_equals_without_whitespace() {
        assert_eq!(Operator::new("="), Ok((Operator::Equal, "")));
    }

    #[test]
    fn parse_equals_equals_without_whitespace() {
        assert_eq!(Operator::new("=="), Ok((Operator::EqualEqual, "")));
    }

    #[test]
    fn parse_not_equals_without_whitespace() {
        assert_eq!(Operator::new("!="), Ok((Operator::BangEquals, "")));
    }

    #[test]
    fn parse_greater_without_whitespace() {
        assert_eq!(Operator::new(">"), Ok((Operator::Greater, "")));
    }

    #[test]
    fn parse_greater_equal_without_whitespace() {
        assert_eq!(Operator::new(">="), Ok((Operator::GreaterEqual, "")));
    }

    #[test]
    fn parse_less_without_whitespace() {
        assert_eq!(Operator::new("<"), Ok((Operator::Less, "")));
    }

    #[test]
    fn parse_less_equal_without_whitespace() {
        assert_eq!(Operator::new("<="), Ok((Operator::LessEqual, "")));
    }

    #[test]
    fn parse_not_without_whitespace() {
        assert_eq!(Operator::new("!"), Ok((Operator::Not, "")));
    }

    #[test]
    fn parse_and_without_whitespace() {
        assert_eq!(Operator::new("&&"), Ok((Operator::And, "")));
    }

    #[test]
    fn parse_or_without_whitespace() {
        assert_eq!(Operator::new("||"), Ok((Operator::Or, "")));
    }

    #[test]
    #[should_panic(expected = "Illegal Operator: @")]
    fn parse_illegal_operator() {
        match Operator::new("@") {
            Ok(res) => res,
            Err(e) => panic!("{}", e),
        };
    }
}
