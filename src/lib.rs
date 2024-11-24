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
    use operator::{Delimeter, Operator};
    use utils::{extract_next_ident, extract_operator_and_delimiter, tag};
    use val::Val;

    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_next_digit("1+2"), ("1", "+2"));
    }

    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_next_digit("10-20"), ("10", "-20"));
    }

    /* Some edge case testing */
    #[test]
    fn extract_nothing_from_empty_input() {
        assert_eq!(extract_next_digit(""), ("", ""));
    }

    #[test]
    fn extract_single_digitr() {
        assert_eq!(extract_next_digit("100"), ("100", ""));
    }

    #[test]
    fn extract_plus() {
        assert_eq!(extract_operator_and_delimiter("+2"), ("+", "2"));
    }

    #[test]
    fn extract_opreator_nothing() {
        assert_eq!(extract_operator_and_delimiter(""), ("", ""));
    }

    #[test]
    fn extract_minus() {
        assert_eq!(extract_operator_and_delimiter("-10"), ("-", "10"));
    }

    #[test]
    fn extract_star() {
        assert_eq!(extract_operator_and_delimiter("*3"), ("*", "3"));
    }

    #[test]
    fn extract_slash() {
        assert_eq!(extract_operator_and_delimiter("/4"), ("/", "4"));
    }
    #[test]
    fn extract_equals_equals() {
        assert_eq!(extract_operator_and_delimiter("==5"), ("==", "5"));
    }

    #[test]
    fn extract_not_equals() {
        assert_eq!(extract_operator_and_delimiter("!=6"), ("!=", "6"));
    }

    #[test]
    fn extract_less_than_equals() {
        assert_eq!(extract_operator_and_delimiter("<=7"), ("<=", "7"));
    }

    #[test]
    fn extract_greater_than_equals() {
        assert_eq!(extract_operator_and_delimiter(">=8"), (">=", "8"));
    }

    #[test]
    fn extract_exclamation() {
        assert_eq!(extract_operator_and_delimiter("!9"), ("!", "9"));
    }

    #[test]
    fn extract_equals() {
        assert_eq!(extract_operator_and_delimiter("=10"), ("=", "10"));
    }

    #[test]
    fn extract_less_than() {
        assert_eq!(extract_operator_and_delimiter("<11"), ("<", "11"));
    }

    #[test]
    fn extract_greater_than() {
        assert_eq!(extract_operator_and_delimiter(">12"), (">", "12"));
    }

    #[test]
    fn extract_percent() {
        assert_eq!(extract_operator_and_delimiter("%13"), ("%", "13"));
    }

    #[test]
    fn extract_left_paren() {
        assert_eq!(
            Delimeter::new("  (4 + 4)"),
            (Delimeter::LeftParen, "4 + 4)")
        );
    }

    #[test]
    fn extract_right_paren() {
        assert_eq!(Delimeter::new("  )"), (Delimeter::RightParen, ""));
    }

    #[test]
    fn extract_left_brace() {
        assert_eq!(Delimeter::new(" {"), (Delimeter::LeftBrace, ""));
    }

    #[test]
    fn extract_right_brace() {
        assert_eq!(Delimeter::new(" }"), (Delimeter::RightBrace, ""));
    }

    #[test]
    fn extract_comma() {
        assert_eq!(Delimeter::new(" ,4"), (Delimeter::Comma, "4"));
    }

    #[test]
    fn extract_dot() {
        assert_eq!(Delimeter::new(" .1"), (Delimeter::Dot, "1"));
    }

    #[test]
    fn extract_alphanumeric_ident() {
        assert_eq!(extract_next_ident("diya = 20"), ("diya", "= 20"));
    }

    #[test]
    fn extract_alphanumeric_ident_with_whitespace() {
        assert_eq!(extract_next_ident("  mikail   = 20"), ("mikail", "= 20"));
    }

    #[test]
    fn extract_alphanumeric_ident_with_number() {
        assert_eq!(extract_next_ident("saad10 = 20"), ("saad10", "= 20"));
    }

    #[test]
    fn extract_alphanumeric_ident_with_number_and_whitespace() {
        assert_eq!(extract_next_ident("  saad10   = 20"), ("saad10", "= 20"));
    }
    #[test]
    fn tag_word() {
        assert_eq!(tag("let", "let a"), Ok(" a"));
    }

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), Ok((Number(123), "")));
    }

    #[test]
    fn parse_add() {
        assert_eq!(Operator::new("+"), Ok((Operator::Add, "")));
    }

    #[test]
    fn parse_subtract() {
        assert_eq!(Operator::new("-"), Ok((Operator::Sub, "")));
    }

    #[test]
    fn parse_multiply() {
        assert_eq!(Operator::new("*"), Ok((Operator::Mul, "")));
    }

    #[test]
    fn parse_divide() {
        assert_eq!(Operator::new("/"), Ok((Operator::Div, "")));
    }

    #[test]
    fn parse_modulus() {
        assert_eq!(Operator::new("%"), Ok((Operator::Mod, "")));
    }

    #[test]
    fn parse_expression_single_number() {
        assert_eq!(
            Expression::new("1+2"),
            Ok((
                Expression::Operation {
                    first_operand: Number(1),
                    operator: Operator::Add,
                    second_operand: Number(2),
                },
                ""
            ))
        );
    }

    #[test]
    fn parse_expression_any_number_one() {
        assert_eq!(
            Expression::new("1333+2"),
            Ok((
                Expression::Operation {
                    first_operand: Number(1333),
                    operator: Operator::Add,
                    second_operand: Number(2),
                },
                ""
            ))
        );
    }

    #[test]
    fn parse_expression_any_number_two() {
        assert_eq!(
            Expression::new("1333+243"),
            Ok((
                Expression::Operation {
                    first_operand: Number(1333),
                    operator: Operator::Add,
                    second_operand: Number(243),
                },
                ""
            ))
        );
    }

    #[test]
    fn parse_number_with_whitespace() {
        assert_eq!(Number::new("  123  "), Ok((Number(123), "")));
    }

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
    fn parse_equals_with_whitespace() {
        assert_eq!(Operator::new("  =  "), Ok((Operator::Equal, "")));
    }

    #[test]
    fn parse_equals_equals_with_whitespace() {
        assert_eq!(Operator::new("  ==  "), Ok((Operator::EqualEqual, "")));
    }

    #[test]
    fn parse_not_equals_with_whitespace() {
        assert_eq!(Operator::new("  !=  "), Ok((Operator::BangEquals, "")));
    }

    #[test]
    fn parse_exclamation_with_whitespace() {
        assert_eq!(Operator::new("  !  "), Ok((Operator::Bang, "")));
    }

    #[test]
    fn parse_greater_with_whitespace() {
        assert_eq!(Operator::new("  >  "), Ok((Operator::Greater, "")));
    }

    #[test]
    fn parse_greater_equal_with_whitespace() {
        assert_eq!(Operator::new("  >=  "), Ok((Operator::GreaterEqual, "")));
    }

    #[test]
    fn parse_less_with_whitespace() {
        assert_eq!(Operator::new("  <  "), Ok((Operator::Less, "")));
    }

    #[test]
    fn parse_less_equal_with_whitespace() {
        assert_eq!(Operator::new("  <=  "), Ok((Operator::LessEqual, "")));
    }

    #[test]
    fn parse_expression_single_number_with_whitespace() {
        assert_eq!(
            Expression::new(" 1  +  2 "),
            Ok((
                Expression::Operation {
                    first_operand: Number(1),
                    operator: Operator::Add,
                    second_operand: Number(2),
                },
                ""
            ))
        );
    }

    #[test]
    fn parse_expression_any_number_one_with_whitespace() {
        assert_eq!(
            Expression::new("  1333  +  2  "),
            Ok((
                Expression::Operation {
                    first_operand: Number(1333),
                    operator: Operator::Add,
                    second_operand: Number(2),
                },
                ""
            ))
        );
    }

    #[test]
    fn parse_expression_any_number_two_with_whitespace() {
        assert_eq!(
            Expression::new("  1333  +  243  "),
            Ok((
                Expression::Operation {
                    first_operand: Number(1333),
                    operator: Operator::Add,
                    second_operand: Number(243),
                },
                ""
            ))
        );
    }

    #[test]
    fn parse_binding_def() {
        assert_eq!(
            BindingDef::new("let a = 10 / 2"),
            Ok((
                BindingDef {
                    name: "a".to_string(),
                    val: Expression::Operation {
                        first_operand: Number(10),
                        operator: Operator::Div,
                        second_operand: Number(2),
                    }
                },
                ""
            ))
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
