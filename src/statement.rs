use crate::{expression::Expression, token::Token};

#[derive(Debug, PartialEq)]
pub enum Statement {
    AssignStatement {
        identifier: Token,
        value: Expression,
    },
    IfStatement,
    WhileStatement,
    ReturnStatement,
}

impl Statement {
    pub fn new(tokens: &[Token]) -> Result<(Self, usize), String> {
        if tokens.is_empty() {
            return Err("No tokens provided".into());
        }

        match tokens[0] {
            Token::Let => {
                if tokens.len() >= 4 {
                    let identifier = match &tokens[1] {
                        Token::Identifier(_) => tokens[1].clone(),
                        _ => {
                            return Err("Assignment statement must start with an identifier".into())
                        }
                    };

                    if tokens[2] != Token::Equal {
                        return Err(
                            "Expected '=' after the identifier in assignment statement".into()
                        );
                    }

                    let expr = Expression::new(&tokens[3..])?;
                    println!("{:?}", expr);

                    return Ok((
                        Statement::AssignStatement {
                            identifier: identifier,
                            value: expr.0,
                        },
                        expr.1,
                    ));
                } else {
                    return Err("Syntax error, expected an assignment statement ".into());
                }
            }
            _ => return Err("Cooked".into()),
        }
    }
}
