use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Token),
    Variable(String),
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Unary {
        operator: Token,
        operand: Box<Expression>,
    },
    Grouping(Box<Expression>),
}
