use crate::operator::Operator;
use crate::{types::Type, utils::extract_whitespace};

#[derive(Debug)]
pub struct Binary {
    pub left_operand: Type,
    pub operator: Operator,
    pub right_operand: Type,
}
impl Binary {
    pub fn new(s: &str) -> Result<(Self, &str), String> {
        let (left_operand, rest) = match Type::new(s) {
            Ok(res) => res,
            Err(e) => {
                return Err(format!("{}", e));
            }
        };

        let (_, rest) = extract_whitespace(rest);

        let (operator, rest) = match Operator::new(rest) {
            Ok(res) => res,
            Err(e) => {
                return Err(format!("{}", e));
            }
        };

        let (_, rest) = extract_whitespace(rest);

        let (right_operand, rest) = match Type::new(rest) {
            Ok(res) => res,
            Err(e) => {
                return Err(format!("{}", e));
            }
        };

        let binary = Binary {
            left_operand,
            operator,
            right_operand,
        };
        return Ok((binary, rest));
    }
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub operands: Vec<Type>,
    pub operators: Vec<Operator>,
}

impl Expression {
    pub fn new(s: &str) -> Result<(Self, &str), String> {
        let mut operands = Vec::new();
        let mut operators = Vec::new();
        let mut remaining = s.trim();

        while !remaining.is_empty() {
            let (operand, rest) = match Type::new(remaining) {
                Ok(res) => res,
                Err(e) => {
                    return Err(format!("{}", e));
                }
            };
            operands.push(operand);

            let (_, rest) = extract_whitespace(rest);

            if rest.is_empty() {
                remaining = rest.trim();
                break;
            }

            let (operator, rest) = match Operator::new(rest.trim()) {
                Ok(res) => res,
                Err(e) => {
                    return Err(format!("Error parsing operator: {}", e));
                }
            };

            operators.push(operator);

            remaining = rest.trim();
        }

        if operands.len() <= operators.len() {
            return Err(format!("Invalid expression: '{}'", s));
        }

        Ok((
            Self {
                operands,
                operators,
            },
            remaining,
        ))
    }

    pub fn eval(&self) -> Type {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::expression::*;
    use crate::types::Type;

    #[test]
    fn parse_simple_arithmetic_expression() {
        let (expression, remaining) = Expression::new("1333+2").unwrap();
        assert_eq!(
            expression,
            Expression {
                operands: vec![Type::Number(1333), Type::Number(2)],
                operators: vec![Operator::Add],
            }
        );
        assert_eq!(remaining, "");
    }

    #[test]
    fn parse_arithmetic_expression_with_whitespace() {
        let (expression, remaining) = Expression::new(" 1  +  2 ").unwrap();
        assert_eq!(
            expression,
            Expression {
                operands: vec![Type::Number(1), Type::Number(2)],
                operators: vec![Operator::Add],
            }
        );
        assert_eq!(remaining, "");
    }

    #[test]
    fn parse_mixed_expression() {
        let (expression, remaining) = Expression::new("1 + 2 < 5 && true").unwrap();
        assert_eq!(
            expression,
            Expression {
                operands: vec![
                    Type::Number(1),
                    Type::Number(2),
                    Type::Number(5),
                    Type::Boolean(true),
                ],
                operators: vec![Operator::Add, Operator::Less, Operator::And,],
            }
        );
        assert_eq!(remaining, "");
    }

    #[test]
    fn parse_expression_with_one_operand() {
        let (expression, remaining) = Expression::new("1").unwrap();
        assert_eq!(
            expression,
            Expression {
                operands: vec![Type::Number(1),],
                operators: vec![],
            }
        );
        assert_eq!(remaining, "");
    }

    #[test]
    fn parse_invalid_operator_expression() {
        let result = Expression::new("1 + @ 3");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unexpected token: @ 3");
    }

    #[test]
    fn parse_invalid_expression_with_extra_operator() {
        let result = Expression::new("+ 2");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unexpected token: + 2");
    }

    #[test]
    fn parse_expression_with_missing_operand() {
        let result = Expression::new("1 + ");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid expression: '1 + '");
    }

    #[test]
    fn test_binary_new_valid_input() {
        let input = "5 + 3";
        let result = Binary::new(input);
        assert!(result.is_ok());
        let (binary, rest) = result.unwrap();

        assert_eq!(binary.left_operand, Type::Number(5));
        assert_eq!(binary.operator, Operator::Add);
        assert_eq!(binary.right_operand, Type::Number(3));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_binary_new_with_whitespace() {
        let input = "  8   *   4  ";
        let result = Binary::new(input);
        assert!(result.is_ok());
        let (binary, rest) = result.unwrap();
        assert_eq!(binary.left_operand, Type::Number(8));
        assert_eq!(binary.operator, Operator::Mul);
        assert_eq!(binary.right_operand, Type::Number(4));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_binary_new_invalid_operator() {
        let input = "10 ? 2";
        let result = Binary::new(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Illegal Operator: ?");
    }

    #[test]
    fn test_binary_new_missing_right_operand() {
        let input = "7 +";
        let result = Binary::new(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unexpected token: ");
    }

    #[test]
    fn test_binary_new_invalid_left_operand() {
        let input = "foo + 10";
        let (result, rest) = Binary::new(input).unwrap();
        assert_eq!(result.left_operand, Type::Identifier("foo".to_string()));
        assert_eq!(result.operator, Operator::Add);
        assert_eq!(result.right_operand, Type::Number(10));
        assert_eq!(rest, "");
    }
}
