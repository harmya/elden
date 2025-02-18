use crate::{expression::Expression, token::Token};

#[derive(Debug, PartialEq)]
pub enum Statement {
    AssignStatement {
        identifier: Token,
        value: Expression,
    },
    IfStatement,
    WhileStatement,
    ReturnStatement {
        value: Expression,
    },
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

                    let expr = match Expression::new(&tokens[3..]) {
                        Ok(expression) => expression,
                        Err(err) => return Err(err),
                    };

                    return Ok((
                        Statement::AssignStatement {
                            identifier,
                            value: expr.0,
                        },
                        expr.1,
                    ));
                } else {
                    return Err("Syntax error, expected an assignment statement ".into());
                }
            }
            Token::Return => {
                let expr = match Expression::new(&tokens[1..]) {
                    Ok(expression) => expression,
                    Err(err) => return Err(err),
                };
                return Ok((Statement::ReturnStatement { value: expr.0 }, expr.1));
            }
            _ => return Err("Expected a statement".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::Expression;
    use crate::token::Token;

    // ----- Assignment Statement Tests -----

    #[test]
    fn test_assignment_statement_success() {
        // let x = 42

        let tokens = vec![
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Equal,
            Token::Number(42),
        ];
        let expected = Statement::AssignStatement {
            identifier: Token::Identifier("x".to_string()),
            value: Expression::Token(Token::Number(42)),
        };

        let result = Statement::new(&tokens);
        assert_eq!(result, Ok((expected, 1)));
    }

    #[test]
    fn test_assignment_missing_equal() {
        //  let x + 42  (error: expected '=' after identifier)
        let tokens = vec![
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Add, // wrong token instead of '='
            Token::Number(42),
        ];
        let result = Statement::new(&tokens);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Expected '=' after the identifier in assignment statement".to_string()
        );
    }

    #[test]
    fn test_assignment_non_identifier() {
        //  let 42 = 42  (error: identifier expected after let)
        let tokens = vec![
            Token::Let,
            Token::Number(42), // not an identifier
            Token::Equal,
            Token::Number(42),
        ];
        let result = Statement::new(&tokens);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Assignment statement must start with an identifier".to_string()
        );
    }

    #[test]
    fn test_assignment_not_enough_tokens() {
        // Tokens: let x  (error: not enough tokens for an assignment)
        let tokens = vec![Token::Let, Token::Identifier("x".to_string())];
        let result = Statement::new(&tokens);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Syntax error, expected an assignment statement ".to_string()
        );
    }

    // ----- Return Statement Tests -----
    #[test]
    fn test_return_statement_success() {
        // Tokens: return 42
        let tokens = vec![Token::Return, Token::Number(42)];
        let expected = Statement::ReturnStatement {
            value: Expression::Token(Token::Number(42)),
        };
        // Expression::new for [Token::Number(42)] returns consumed count of 1.
        let result = Statement::new(&tokens);
        assert_eq!(result, Ok((expected, 1)));
    }

    #[test]
    fn test_return_statement_no_expression() {
        // Tokens: return  (error: missing expression after return)
        let tokens = vec![Token::Return];
        let result = Statement::new(&tokens);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Expected an expression".to_string());
    }

    // ----- General Statement Error Tests -----

    #[test]
    fn test_statement_no_tokens() {
        let tokens: Vec<Token> = vec![];
        let result = Statement::new(&tokens);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No tokens provided".to_string());
    }

    #[test]
    fn test_statement_invalid_start() {
        // Tokens: x (an identifier, not starting with let or return)
        let tokens = vec![Token::Identifier("x".to_string())];
        let result = Statement::new(&tokens);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Expected a statement".to_string());
    }
}
