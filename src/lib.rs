use utils::{extract_next_digits, extract_operator, extract_whitespace};

pub mod utils;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);
impl Number {
    pub fn new(s: &str) -> (Self, &str) {
        let (number, rest) = extract_next_digits(s.trim());
        (Self(number.parse().unwrap()), rest)
    }
}

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
        let (operator, rest) = extract_operator(s.trim());
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
pub struct Expression {
    pub first_operand: Number,
    pub second_operand: Number,
    pub operator: Operator,
}

impl Expression {
    pub fn new(s: &str) -> (Self, &str) {
        let (first_operand, rest) = Number::new(s.trim());
        let (_, rest) = extract_whitespace(rest);

        let (operator, rest) = Operator::new(rest.trim());
        let (_, rest) = extract_whitespace(rest);

        let (second_operand, rest) = Number::new(rest.trim());
        (
            Self {
                first_operand,
                second_operand,
                operator,
            },
            rest,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), (Number(123), ""));
    }

    #[test]
    fn parse_add() {
        assert_eq!(Operator::new("+"), (Operator::Add, ""));
    }

    #[test]
    fn parse_subtract() {
        assert_eq!(Operator::new("-"), (Operator::Sub, ""));
    }

    #[test]
    fn parse_multiply() {
        assert_eq!(Operator::new("*"), (Operator::Mul, ""));
    }

    #[test]
    fn parse_divide() {
        assert_eq!(Operator::new("/"), (Operator::Div, ""));
    }

    #[test]
    fn parse_modulus() {
        assert_eq!(Operator::new("%"), (Operator::Mod, ""));
    }

    #[test]
    fn parse_expression_single_number() {
        assert_eq!(
            Expression::new("1+2"),
            (
                Expression {
                    first_operand: Number(1),
                    operator: Operator::Add,
                    second_operand: Number(2),
                },
                ""
            )
        );
    }

    #[test]
    fn parse_expression_any_number_one() {
        assert_eq!(
            Expression::new("1333+2"),
            (
                Expression {
                    first_operand: Number(1333),
                    operator: Operator::Add,
                    second_operand: Number(2),
                },
                ""
            )
        );
    }
    #[test]
    fn parse_expression_any_number_two() {
        assert_eq!(
            Expression::new("1333+243"),
            (
                Expression {
                    first_operand: Number(1333),
                    operator: Operator::Add,
                    second_operand: Number(243),
                },
                ""
            )
        );
    }
    #[test]
    fn parse_number_with_whitespace() {
        assert_eq!(Number::new("  123  "), (Number(123), ""));
    }

    #[test]
    fn parse_add_with_whitespace() {
        assert_eq!(Operator::new("  +  "), (Operator::Add, ""));
    }

    #[test]
    fn parse_subtract_with_whitespace() {
        assert_eq!(Operator::new("  -  "), (Operator::Sub, ""));
    }

    #[test]
    fn parse_multiply_with_whitespace() {
        assert_eq!(Operator::new("  *  "), (Operator::Mul, ""));
    }

    #[test]
    fn parse_divide_with_whitespace() {
        assert_eq!(Operator::new("  /  "), (Operator::Div, ""));
    }

    #[test]
    fn parse_modulus_with_whitespace() {
        assert_eq!(Operator::new("  %  "), (Operator::Mod, ""));
    }

    #[test]
    fn parse_equals_with_whitespace() {
        assert_eq!(Operator::new("  =  "), (Operator::Equal, ""));
    }

    #[test]
    fn parse_equals_equals_with_whitespace() {
        assert_eq!(Operator::new("  ==  "), (Operator::EqualEqual, ""));
    }

    #[test]
    fn parse_not_equals_with_whitespace() {
        assert_eq!(Operator::new("  !=  "), (Operator::BangEquals, ""));
    }

    #[test]
    fn parse_exclamation_with_whitespace() {
        assert_eq!(Operator::new("  !  "), (Operator::Bang, ""));
    }

    #[test]
    fn parse_greater_with_whitespace() {
        assert_eq!(Operator::new("  >  "), (Operator::Greater, ""));
    }

    #[test]
    fn parse_greater_equal_with_whitespace() {
        assert_eq!(Operator::new("  >=  "), (Operator::GreaterEqual, ""));
    }

    #[test]
    fn parse_less_with_whitespace() {
        assert_eq!(Operator::new("  <  "), (Operator::Less, ""));
    }

    #[test]
    fn parse_less_equal_with_whitespace() {
        assert_eq!(Operator::new("  <=  "), (Operator::LessEqual, ""));
    }

    #[test]
    fn parse_expression_single_number_with_whitespace() {
        assert_eq!(
            Expression::new(" 1  +  2 "),
            (
                Expression {
                    first_operand: Number(1),
                    operator: Operator::Add,
                    second_operand: Number(2),
                },
                ""
            )
        );
    }

    #[test]
    fn parse_expression_any_number_one_with_whitespace() {
        assert_eq!(
            Expression::new("  1333  +  2  "),
            (
                Expression {
                    first_operand: Number(1333),
                    operator: Operator::Add,
                    second_operand: Number(2),
                },
                ""
            )
        );
    }

    #[test]
    fn parse_expression_any_number_two_with_whitespace() {
        assert_eq!(
            Expression::new("  1333  +  243  "),
            (
                Expression {
                    first_operand: Number(1333),
                    operator: Operator::Add,
                    second_operand: Number(243),
                },
                ""
            )
        );
    }
}
