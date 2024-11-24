use utils::extract_next_digit;

mod env;
mod utils;

pub mod binding;
pub mod expression;
pub mod operator;
pub mod val;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);
impl Number {
    pub fn new(s: &str) -> (Self, &str) {
        let (number, rest) = extract_next_digit(s.trim());
        (Self(number.parse().unwrap()), rest)
    }
}

#[cfg(test)]
mod tests {
    use binding::BindingDef;
    use expression::Expression;
    use operator::Operator;
    use val::Val;

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

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::new("let a = 10 / 2"),
            (
                BindingDef {
                    name: "a".to_string(),
                    val: Expression {
                        first_operand: Number(10),
                        operator: Operator::Div,
                        second_operand: Number(2),
                    }
                },
                ""
            ),
        );
    }

    #[test]
    fn eval_add() {
        assert_eq!(
            Expression {
                first_operand: Number(10),
                second_operand: Number(10),
                operator: Operator::Add,
            }
            .eval(),
            Val::Number(20),
        );
    }

    #[test]
    fn eval_sub() {
        assert_eq!(
            Expression {
                first_operand: Number(1),
                second_operand: Number(5),
                operator: Operator::Sub,
            }
            .eval(),
            Val::Number(-4),
        );
    }

    #[test]
    fn eval_mul() {
        assert_eq!(
            Expression {
                first_operand: Number(5),
                second_operand: Number(6),
                operator: Operator::Mul,
            }
            .eval(),
            Val::Number(30),
        );
    }

    #[test]
    fn eval_div() {
        assert_eq!(
            Expression {
                first_operand: Number(200),
                second_operand: Number(20),
                operator: Operator::Div,
            }
            .eval(),
            Val::Number(10),
        );
    }
}
