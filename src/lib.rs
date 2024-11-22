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
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl Op {
    pub fn new(s: &str) -> (Self, &str) {
        let (operator, rest) = extract_operator(s.trim());
        let operator = match operator {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            "%" => Self::Mod,
            _ => panic!("Illegal Operator"),
        };
        (operator, rest)
    }
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub first_operand: Number,
    pub second_operand: Number,
    pub operator: Op,
}

impl Expression {
    pub fn new(s: &str) -> (Self, &str) {
        let (first_operand, rest) = Number::new(s.trim());
        let (_, rest) = extract_whitespace(rest);

        let (operator, rest) = Op::new(rest.trim());
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
        assert_eq!(Op::new("+"), (Op::Add, ""));
    }

    #[test]
    fn parse_subtract() {
        assert_eq!(Op::new("-"), (Op::Sub, ""));
    }

    #[test]
    fn parse_multiply() {
        assert_eq!(Op::new("*"), (Op::Mul, ""));
    }

    #[test]
    fn parse_divide() {
        assert_eq!(Op::new("/"), (Op::Div, ""));
    }

    #[test]
    fn parse_modulus() {
        assert_eq!(Op::new("%"), (Op::Mod, ""));
    }

    #[test]
    fn parse_expression_single_number() {
        assert_eq!(
            Expression::new("1+2"),
            (
                Expression {
                    first_operand: Number(1),
                    operator: Op::Add,
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
                    operator: Op::Add,
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
                    operator: Op::Add,
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
        assert_eq!(Op::new("  +  "), (Op::Add, ""));
    }

    #[test]
    fn parse_subtract_with_whitespace() {
        assert_eq!(Op::new("  -  "), (Op::Sub, ""));
    }

    #[test]
    fn parse_multiply_with_whitespace() {
        assert_eq!(Op::new("  *  "), (Op::Mul, ""));
    }

    #[test]
    fn parse_divide_with_whitespace() {
        assert_eq!(Op::new("  /  "), (Op::Div, ""));
    }

    #[test]
    fn parse_modulus_with_whitespace() {
        assert_eq!(Op::new("  %  "), (Op::Mod, ""));
    }

    #[test]
    fn parse_expression_single_number_with_whitespace() {
        assert_eq!(
            Expression::new(" 1  +  2 "),
            (
                Expression {
                    first_operand: Number(1),
                    operator: Op::Add,
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
                    operator: Op::Add,
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
                    operator: Op::Add,
                    second_operand: Number(243),
                },
                ""
            )
        );
    }
}
