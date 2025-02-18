use crate::token::{self, Token};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Token(Token),
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

fn parse_logical_or(tokens: &[Token]) -> Result<(Expression, usize), String> {
    println!("tokens in expr {:?}", tokens);
    let mut consumed = 0;
    let mut left = Expression::Token(tokens[consumed].clone());
    consumed += 1;
    while consumed < tokens.len() {
        if tokens[consumed] == Token::Or {
            let op = tokens[consumed].clone();
            let right = Expression::Token(tokens[consumed + 1].clone());
            left = Expression::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
            consumed += 2;
        } else {
            break;
        }
    }
    Ok((left, consumed))
}

impl Expression {
    pub fn new(tokens: &[Token]) -> Result<(Self, usize), String> {
        if tokens.is_empty() {
            return Err("Expected an expression".into());
        }

        // Start at the lowest precedence level: logical OR.
        parse_logical_or(tokens)
    }
}
