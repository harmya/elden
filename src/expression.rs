use crate::{operator::*, types::Type, utils::extract_whitespace};

#[derive(Debug, PartialEq)]
pub enum ExpressionType {
    Arithmetic,
    Relational,
    Logical,
    Empty,
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub operands: Vec<Type>,
    pub operators: Vec<Operator>,
    pub expression_type: ExpressionType,
}

impl Expression {
    fn validate_expression_type(
        operator: &Operator,
        prev_expression_type: ExpressionType,
    ) -> ExpressionType {
        let current_expression_type = match operator {
            Operator::Arithmetic(_) => ExpressionType::Arithmetic,
            Operator::Relational(_) => ExpressionType::Relational,
            Operator::Logical(_) => ExpressionType::Logical,
            _ => panic!("Illegal use of operator"),
        };

        if prev_expression_type == ExpressionType::Empty {
            return current_expression_type;
        } else if current_expression_type == prev_expression_type {
            prev_expression_type
        } else {
            panic!(
                "Mismatched operator type: expected {:?}, found {:?}",
                prev_expression_type, current_expression_type
            );
        }
    }

    pub fn new(s: &str) -> (Self, &str) {
        let mut expression_type: ExpressionType = ExpressionType::Empty;
        let mut operands = Vec::new();
        let mut operators = Vec::new();
        let mut remaining = s.trim();

        while !remaining.is_empty() {
            let (operand, rest) = Type::new(remaining);
            let (_, rest) = extract_whitespace(rest);
            let (operator, rest) = Operator::new(rest.trim());
            expression_type = Self::validate_expression_type(&operator, expression_type);
            remaining = rest.trim();

            operands.push(operand);
            operators.push(operator);
        }

        if operands.len() <= operators.len() {
            panic!("Invalid expression: '{}'", s);
        }

        (
            Self {
                operands,
                operators,
                expression_type,
            },
            remaining,
        )
    }

    pub fn eval(&self) -> Type {
        todo!()
    }
}
