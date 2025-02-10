use crate::operator::{self, Operator};
use crate::{types::Type, utils::extract_whitespace};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Type),
    Variable(String),
    Binary {
        left: Box<Expression>,
        operator: Operator,
        right: Box<Expression>,
    },
    Unary {
        operator: Operator,
        operand: Box<Expression>,
    },
    Grouping(Box<Expression>),
}
