use std::ffi::c_int;

use crate::{
    expression::Expression,
    token::{self, Token},
};

#[derive(Debug, PartialEq)]
pub enum Statement {
    AssignStatement {
        identifier: Token,
        value: Expression,
    },
    IfStatement {
        cond: Expression,
        if_then: Vec<Statement>,
        else_then: Option<Vec<Statement>>,
    },
    WhileStatement {
        cond: Expression,
        loop_stmt: Vec<Statement>,
    },
    ReturnStatement {
        value: Expression,
    },
}

pub fn get_statement_slice(
    tokens: &[Token],
    curr_index: usize,
) -> Result<(&[Token], usize), String> {
    // Collect tokens until a semicolon is found
    let mut index = curr_index;
    while index < tokens.len() && tokens[index] != Token::SemiColon {
        index += 1;
    }
    // If we've reached the end without finding a semicolon, it's a syntax error
    if index >= tokens.len() {
        return Err("Syntax error, expected semicolon at end of statement in function body".into());
    }
    // Slice containing tokens for the current statement
    let statement_tokens = &tokens[curr_index..=index];
    Ok((statement_tokens, index + 1))
}

fn parse_block(tokens: &[Token]) -> Result<(Vec<Statement>, usize), String> {
    // Assume the block starts with '{'
    if tokens.is_empty() || tokens[0] != Token::LeftBrace {
        return Err("Expected '{' to start block".into());
    }
    let mut statements = Vec::new();
    let mut curr_index = 1; // Skip '{'
    while curr_index < tokens.len() && tokens[curr_index] != Token::RightBrace {
        let (stmt, consumed) = Statement::new(&tokens[curr_index..])?;
        statements.push(stmt);
        curr_index += consumed;
    }
    if curr_index >= tokens.len() || tokens[curr_index] != Token::RightBrace {
        return Err("Expected '}' at end of block".into());
    }
    curr_index += 1; // Consume '}'
    Ok((statements, curr_index))
}

impl Statement {
    pub fn new(tokens: &[Token]) -> Result<(Self, usize), String> {
        if tokens.is_empty() {
            return Err("No tokens provided".into());
        }

        match tokens[0] {
            Token::Let => {
                // We assume the statement is of the form:
                // let Identifier, Equal, <expression>, SemiColon
                let (token_slice, consumed) = match get_statement_slice(tokens, 0) {
                    Ok(output) => output,
                    Err(e) => return Err(e),
                };

                if token_slice.len() >= 4 {
                    let identifier = match &token_slice[1] {
                        Token::Identifier(_) => token_slice[1].clone(),
                        _ => {
                            return Err("Assignment statement must start with an identifier".into())
                        }
                    };

                    if token_slice[2] != Token::Equal {
                        return Err(
                            "Expected '=' after the identifier in assignment statement".into()
                        );
                    }

                    let expr = Expression::new(&token_slice[3..token_slice.len() - 1])?;

                    return Ok((
                        Statement::AssignStatement {
                            identifier,
                            value: expr.0,
                        },
                        consumed,
                    ));
                } else {
                    return Err("Syntax error, expected an assignment statement ".into());
                }
            }
            Token::Identifier(_) => {
                // We assume the statement is of the form:
                // Identifier, Equal, <expression>, SemiColon
                let (token_slice, consumed) = get_statement_slice(tokens, 0)?;
                if token_slice.len() >= 3 {
                    let identifier = token_slice[0].clone();
                    if token_slice[1] != Token::Equal {
                        return Err(
                            "Expected '=' after the identifier in assignment statement".into()
                        );
                    }
                    let expr = Expression::new(&token_slice[2..token_slice.len() - 1])?;
                    return Ok((
                        Statement::AssignStatement {
                            identifier,
                            value: expr.0,
                        },
                        consumed,
                    ));
                } else {
                    return Err("Syntax error, expected an assignment statement".into());
                }
            }
            Token::Return => {
                //now, since the first token is a return, we get a slice until the next semi colon
                let (token_slice, consumed) = match get_statement_slice(&tokens, 0) {
                    Ok(output) => output,
                    Err(e) => return Err(e),
                };

                let expr = Expression::new(&token_slice[1..token_slice.len() - 1])?;
                return Ok((Statement::ReturnStatement { value: expr.0 }, consumed));
            }
            Token::If => {
                // If, LeftParen, <condition tokens>, RightParen, LeftBrace, <if body tokens>, RightBrace,
                // optionally Else, LeftBrace, <else body tokens>, RightBrace
                if tokens.len() > 2 && tokens.get(1) == Some(&Token::LeftParen) {
                    // Find the matching right parenthesis for the condition
                    let mut paren_depth = 0;
                    let mut right_paren_index = None;
                    for i in 1..tokens.len() {
                        match tokens[i] {
                            Token::LeftParen => {
                                paren_depth += 1;
                            }
                            Token::RightParen => {
                                paren_depth -= 1;
                                if paren_depth == 0 {
                                    right_paren_index = Some(i);
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    let right_paren_index = match right_paren_index {
                        Some(i) => i,
                        None => {
                            return Err(
                                "Syntax error, expected closing parentheses for if statement"
                                    .into(),
                            )
                        }
                    };

                    // Parse the condition from tokens[2..right_paren_index]
                    let (cond_expr, _cond_consumed) =
                        Expression::new(&tokens[2..right_paren_index])?;

                    // After the condition, expect a left brace for the if-block
                    if tokens.get(right_paren_index + 1) != Some(&Token::LeftBrace) {
                        return Err("Syntax error, expected '{' after if condition".into());
                    }
                    // use parse_block to parse statements enclosed in {}
                    let (if_body, body_consumed) = parse_block(&tokens[right_paren_index + 1..])?;
                    let mut curr_index = right_paren_index + 1 + body_consumed;

                    // Check for an optional 'else' block
                    let mut else_body = None;
                    if tokens.get(curr_index) == Some(&Token::Else) {
                        curr_index += 1; // consume 'else'
                        if tokens.get(curr_index) == Some(&Token::If) {
                            // This is an 'else if' so parse it as a nested if-statement
                            let (nested_if, consumed_nested) =
                                Statement::new(&tokens[curr_index..])?;
                            curr_index += consumed_nested;
                            else_body = Some(vec![nested_if]);
                        } else if tokens.get(curr_index) == Some(&Token::LeftBrace) {
                            // else block: parse the block
                            let (else_stmts, else_consumed) = parse_block(&tokens[curr_index..])?;
                            else_body = Some(else_stmts);
                            curr_index += else_consumed;
                        } else {
                            return Err("Syntax error, expected '{' or 'if' after else".into());
                        }
                    }

                    Ok((
                        Statement::IfStatement {
                            cond: cond_expr,
                            if_then: if_body,
                            else_then: else_body,
                        },
                        curr_index,
                    ))
                } else {
                    Err("Syntax error, expected opening parentheses for if statement".into())
                }
            }

            Token::While => {
                // Parse while statement:
                // while, LeftParen, <condition tokens>, RightParen, LeftBrace, <loop body>, RightBrace
                if tokens.len() > 2 && tokens.get(1) == Some(&Token::LeftParen) {
                    let mut paren_depth = 0;
                    let mut right_paren_index = None;
                    for i in 1..tokens.len() {
                        match tokens[i] {
                            Token::LeftParen => paren_depth += 1,
                            Token::RightParen => {
                                paren_depth -= 1;
                                if paren_depth == 0 {
                                    right_paren_index = Some(i);
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    let right_paren_index =
                        match right_paren_index {
                            Some(i) => i,
                            None => return Err(
                                "Syntax error, expected closing parenthesis for while statement"
                                    .into(),
                            ),
                        };

                    // Parse condition from tokens[2..right_paren_index]
                    let (cond_expr, _cond_consumed) =
                        Expression::new(&tokens[2..right_paren_index])?;

                    // Expect a left brace after the condition for the loop block.
                    if tokens.get(right_paren_index + 1) != Some(&Token::LeftBrace) {
                        return Err("Syntax error, expected '{' after while condition".into());
                    }

                    let (loop_stmts, body_consumed) =
                        parse_block(&tokens[right_paren_index + 1..])?;
                    let curr_index = right_paren_index + 1 + body_consumed;
                    Ok((
                        Statement::WhileStatement {
                            cond: cond_expr,
                            loop_stmt: loop_stmts,
                        },
                        curr_index,
                    ))
                } else {
                    Err("Syntax error, expected opening parenthesis for while statement".into())
                }
            }
            _ => Err("Expected a statement".into()),
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
            Token::SemiColon,
        ];
        let expected = Statement::AssignStatement {
            identifier: Token::Identifier("x".to_string()),
            value: Expression::Token(Token::Number(42)),
        };

        let result = Statement::new(&tokens);
        assert_eq!(result, Ok((expected, 5)));
    }

    #[test]
    fn test_assignment_missing_equal() {
        //  let x + 42  (error: expected '=' after identifier)
        let tokens = vec![
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Add, // wrong token instead of '='
            Token::Number(42),
            Token::SemiColon,
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
            Token::SemiColon,
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
            "Syntax error, expected semicolon at end of statement in function body".to_string()
        );
    }

    // ----- Return Statement Tests -----
    #[test]
    fn test_return_statement_success() {
        // Tokens: return 42
        let tokens = vec![Token::Return, Token::Number(42), Token::SemiColon];
        let expected = Statement::ReturnStatement {
            value: Expression::Token(Token::Number(42)),
        };
        // Expression::new for [Token::Number(42)] returns consumed count of 1.
        let result = Statement::new(&tokens);
        assert_eq!(result, Ok((expected, 3)));
    }

    #[test]
    fn test_return_statement_no_expression() {
        // Tokens: return  (error: missing expression after return)
        let tokens = vec![Token::Return, Token::SemiColon];
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
        let tokens = vec![Token::Identifier("x".to_string()), Token::SemiColon];
        let result = Statement::new(&tokens);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Syntax error, expected an assignment statement".to_string()
        );
    }
    #[test]
    fn test_if_statement_without_else() {
        // This represents:
        // if (x) { return 1; }
        // Token layout:
        // [If, LeftParen, Identifier("x"), RightParen, LeftBrace,
        //  Return, Number(1), SemiColon, RightBrace]
        let tokens = vec![
            Token::If,
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Return,
            Token::Number(1),
            Token::SemiColon,
            Token::RightBrace,
        ];

        let expected = Statement::IfStatement {
            cond: Expression::Token(Token::Identifier("x".to_string())),
            if_then: vec![Statement::ReturnStatement {
                value: Expression::Token(Token::Number(1)),
            }],
            else_then: None,
        };

        let result = Statement::new(&tokens);
        assert_eq!(result, Ok((expected, tokens.len())));
    }

    #[test]
    fn test_if_statement_with_else() {
        // This represents:
        // if (x) { return 1; } else { return 2; }
        // Token layout:
        // [If, LeftParen, Identifier("x"), RightParen, LeftBrace,
        //  Return, Number(1), SemiColon, RightBrace,
        //  Else, LeftBrace, Return, Number(2), SemiColon, RightBrace]
        let tokens = vec![
            Token::If,
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Return,
            Token::Number(1),
            Token::SemiColon,
            Token::RightBrace,
            Token::Else,
            Token::LeftBrace,
            Token::Return,
            Token::Number(2),
            Token::SemiColon,
            Token::RightBrace,
        ];

        let expected = Statement::IfStatement {
            cond: Expression::Token(Token::Identifier("x".to_string())),
            if_then: vec![Statement::ReturnStatement {
                value: Expression::Token(Token::Number(1)),
            }],
            else_then: Some(vec![Statement::ReturnStatement {
                value: Expression::Token(Token::Number(2)),
            }]),
        };

        let result = Statement::new(&tokens);
        assert_eq!(result, Ok((expected, tokens.len())));
    }

    #[test]
    fn test_if_statement_missing_closing_brace() {
        // This represents an if-statement with a missing closing brace for the if-block:
        // if (x) { return 1;
        let tokens = vec![
            Token::If,
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Return,
            Token::Number(1),
            Token::SemiColon,
            // Missing RightBrace here
        ];
        let result = Statement::new(&tokens);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Expected '}' at end of block".to_string()
        );
    }

    #[test]
    fn test_while_statement_success() {
        // while (x) { return 1; }
        // Token layout:
        // [While, LeftParen, Identifier("x"), RightParen, LeftBrace, Return, Number(1), SemiColon, RightBrace]
        let tokens = vec![
            Token::While,
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Return,
            Token::Number(1),
            Token::SemiColon,
            Token::RightBrace,
        ];

        let expected = Statement::WhileStatement {
            cond: Expression::Token(Token::Identifier("x".to_string())),
            loop_stmt: vec![Statement::ReturnStatement {
                value: Expression::Token(Token::Number(1)),
            }],
        };

        let result = Statement::new(&tokens);
        assert_eq!(result, Ok((expected, tokens.len())));
    }

    #[test]
    fn test_while_statement_missing_paren() {
        // Missing closing parenthesis: while (x { return 1; }
        let tokens = vec![
            Token::While,
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            // Missing RightParen here
            Token::LeftBrace,
            Token::Return,
            Token::Number(1),
            Token::SemiColon,
            Token::RightBrace,
        ];
        let result = Statement::new(&tokens);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Syntax error, expected closing parenthesis for while statement".to_string()
        );
    }

    #[test]
    fn test_while_statement_missing_brace() {
        // Missing left brace after condition: while (x) return 1; }
        let tokens = vec![
            Token::While,
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::RightParen,
            // Missing LeftBrace here
            Token::Return,
            Token::Number(1),
            Token::SemiColon,
            Token::RightBrace,
        ];
        let result = Statement::new(&tokens);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Syntax error, expected '{' after while condition".to_string()
        );
    }

    #[test]
    fn test_while_statement_missing_body_end() {
        // While statement with missing closing brace for loop body:
        // while (x) { return 1;
        let tokens = vec![
            Token::While,
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Return,
            Token::Number(1),
            Token::SemiColon,
            // Missing RightBrace here
        ];
        let result = Statement::new(&tokens);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Expected '}' at end of block".to_string()
        );
    }
}
