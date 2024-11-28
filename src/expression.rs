use crate::{operator::*, utils::extract_whitespace, val::Val, Number};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Arithmetic(ArithmeticExpression),
}

#[derive(Debug, PartialEq)]
pub struct ArithmeticExpression {
    pub first_operand: Number,
    pub second_operand: Number,
    pub operator: Operator,
}

#[derive(Debug, PartialEq)]
pub struct RelationalExpression {
    pub first_operand: Number,
    pub second_operand: Number,
    pub operator: Operator,
}

#[derive(Debug, PartialEq)]
pub struct LogicalExpression {
    pub first_operand: bool,
    pub second_operand: Option<bool>,
    pub operator: Operator,
}

#[derive(Debug, PartialEq)]
pub struct AssignmentExpression {
    pub variable_name: String,
    pub value: Val,
    pub operator: Operator,
}

impl Expression {
    pub fn new(s: &str) -> (Self, &str) {
        if let Ok((arithmetic_expr, rest)) = ArithmeticExpression::try_new(s) {
            return (Self::Arithmetic(arithmetic_expr), rest);
        }

        panic!("Failed to parse expression: {}", s);
    }

    pub fn eval(&self) -> Val {
        match self {
            Expression::Arithmetic(expr) => expr.eval(),
        }
    }
}

impl ArithmeticExpression {
    pub fn try_new(s: &str) -> Result<(Self, &str), String> {
        let (first_operand, rest) = Number::new(s.trim());
        let (_, rest) = extract_whitespace(rest);

        let (operator, rest) = Operator::new(rest.trim());
        let (_, rest) = extract_whitespace(rest);

        let (second_operand, rest) = Number::new(rest.trim());
        Ok((
            Self {
                first_operand,
                second_operand,
                operator,
            },
            rest,
        ))
    }

    pub fn eval(&self) -> Val {
        let Number(first_operand) = self.first_operand;
        let Number(second_operand) = self.second_operand;

        let result = match &self.operator {
            Operator::Arithmetic(op) => match op {
                ArithmeticOperator::Add => first_operand + second_operand,
                ArithmeticOperator::Sub => first_operand - second_operand,
                ArithmeticOperator::Mul => first_operand * second_operand,
                ArithmeticOperator::Div => first_operand / second_operand,
                ArithmeticOperator::Mod => first_operand % second_operand,
            },
            _ => panic!("Invalid operator for ArithmeticExpression"),
        };

        Val::Number(result)
    }
}
#[cfg(test)]
mod tests {
    use crate::expression::*;

    #[test]
    fn parse_arithmetic_expression_simple() {
        assert_eq!(
            ArithmeticExpression::try_new("1333+2").unwrap(),
            (
                ArithmeticExpression {
                    first_operand: Number(1333),
                    operator: Operator::Arithmetic(ArithmeticOperator::Add),
                    second_operand: Number(2),
                },
                ""
            )
        );
    }

    #[test]
    fn parse_arithmetic_expression_with_whitespace() {
        assert_eq!(
            ArithmeticExpression::try_new(" 1  +  2 ").unwrap(),
            (
                ArithmeticExpression {
                    first_operand: Number(1),
                    operator: Operator::Arithmetic(ArithmeticOperator::Add),
                    second_operand: Number(2),
                },
                ""
            )
        );
    }

    #[test]
    fn parse_expression_simple() {
        assert_eq!(
            Expression::new("1333+243"),
            (
                Expression::Arithmetic(ArithmeticExpression {
                    first_operand: Number(1333),
                    operator: Operator::Arithmetic(ArithmeticOperator::Add),
                    second_operand: Number(243),
                }),
                ""
            )
        );
    }

    #[test]
    fn parse_expression_with_whitespace() {
        assert_eq!(
            Expression::new("  1333  +  243  "),
            (
                Expression::Arithmetic(ArithmeticExpression {
                    first_operand: Number(1333),
                    operator: Operator::Arithmetic(ArithmeticOperator::Add),
                    second_operand: Number(243),
                }),
                ""
            )
        );
    }

    #[test]
    #[should_panic]
    fn parse_invalid_expression() {
        assert!(ArithmeticExpression::try_new("abc+def").is_err());
    }

    #[test]
    fn eval_arithmetic_expression_add() {
        let expr = ArithmeticExpression {
            first_operand: Number(10),
            second_operand: Number(15),
            operator: Operator::Arithmetic(ArithmeticOperator::Add),
        };
        assert_eq!(expr.eval(), Val::Number(25));
    }

    #[test]
    fn eval_arithmetic_expression_sub() {
        let expr = ArithmeticExpression {
            first_operand: Number(10),
            second_operand: Number(15),
            operator: Operator::Arithmetic(ArithmeticOperator::Sub),
        };
        assert_eq!(expr.eval(), Val::Number(-5));
    }

    #[test]
    fn eval_arithmetic_expression_mul() {
        let expr = ArithmeticExpression {
            first_operand: Number(5),
            second_operand: Number(6),
            operator: Operator::Arithmetic(ArithmeticOperator::Mul),
        };
        assert_eq!(expr.eval(), Val::Number(30));
    }

    #[test]
    fn eval_arithmetic_expression_div() {
        let expr = ArithmeticExpression {
            first_operand: Number(200),
            second_operand: Number(20),
            operator: Operator::Arithmetic(ArithmeticOperator::Div),
        };
        assert_eq!(expr.eval(), Val::Number(10));
    }

    #[test]
    fn eval_arithmetic_expression_mod() {
        let expr = ArithmeticExpression {
            first_operand: Number(7),
            second_operand: Number(3),
            operator: Operator::Arithmetic(ArithmeticOperator::Mod),
        };
        assert_eq!(expr.eval(), Val::Number(1));
    }
}
