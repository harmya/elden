use crate::token::{self, Token};

#[derive(Debug, PartialEq)]
pub enum Expression {
    ArrayDec {
        arr_expr: Vec<Token>,
    },
    FunctionCall {
        identifier: Token,
        args: Vec<Token>,
    },
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
fn parse_array_dec(tokens: &[Token]) -> Result<(Expression, usize), String> {
    // since first token is a left square bracket
    let mut index = 1;
    let mut args = Vec::new();

    while index < tokens.len() {
        if tokens.get(index) == Some(&Token::SemiColon) {
            return Err("Expected ']' in the array declaration".to_string());
        } else if tokens.get(index) != Some(&Token::Comma) {
            args.push(tokens[index].clone());
        }
        index += 1;
    }

    Ok((Expression::ArrayDec { arr_expr: args }, index))
}
fn parse_function_call(tokens: &[Token]) -> Result<(Expression, usize), String> {
    // We know the first token is an identifier.
    let identifier = tokens[0].clone();
    // The next token must be a left parenthesis.
    if tokens.len() < 2 || tokens[1] != Token::LeftParen {
        return Err("Expected '(' after identifier for function call".to_string());
    }
    let mut consumed = 2; // Consumed the identifier and LeftParen
    let mut args = Vec::new();

    // If the next token is a right parenthesis, it's a call with no arguments
    if tokens.len() > consumed && tokens[consumed] == Token::RightParen {
        consumed += 1;
        return Ok((Expression::FunctionCall { identifier, args }, consumed));
    }
    // Otherwise, parse arguments separated by commas
    loop {
        if consumed >= tokens.len() {
            return Err("Expected ')' in function call".to_string());
        }
        args.push(tokens[consumed].clone());
        consumed += 1;
        if consumed >= tokens.len() {
            return Err("Expected ')' in function call".to_string());
        }
        match tokens[consumed] {
            Token::Comma => {
                consumed += 1;
            }
            Token::RightParen => {
                consumed += 1;
                break;
            }
            _ => {
                return Err(format!("Unexpected token in argument list"));
            }
        }
    }

    Ok((Expression::FunctionCall { identifier, args }, consumed))
}

fn parse_primary(tokens: &[Token]) -> Result<(Expression, usize), String> {
    if tokens.is_empty() {
        return Err("Unexpected end of input while parsing logical AND expression.".to_string());
    }
    match tokens.get(0) {
        //parse if there is a function call
        Some(Token::Identifier(_)) => {
            if tokens.len() > 1 && tokens[1] == Token::LeftParen {
                parse_function_call(tokens)
            } else {
                Ok((Expression::Token(tokens[0].clone()), 1))
            }
        }
        Some(Token::LeftSquare) => parse_array_dec(tokens),
        Some(Token::Number(_)) | Some(Token::StringLiteral(_)) => {
            Ok((Expression::Token(tokens[0].clone()), 1))
        }
        Some(Token::LeftParen) => {
            // Parse an expression inside parentheses.
            let (expr, consumed) = parse_logical_or(&tokens[1..])?;
            if consumed + 1 >= tokens.len() || tokens[consumed + 1] != Token::RightParen {
                return Err("Expected ')' after grouped expression".to_string());
            }
            Ok((Expression::Grouping(Box::new(expr)), consumed + 2))
        }
        _ => return Err("Unexpected end of input while parsing.".to_string()),
    }
}

fn parse_unary(tokens: &[Token]) -> Result<(Expression, usize), String> {
    if tokens.is_empty() {
        return Err("Unexpected end of input while parsing logical unary expression.".to_string());
    }

    if tokens.get(0) == Some(&Token::Not) {
        let operator = tokens[0].clone();
        let (right, right_consumed) = match parse_unary(&tokens[1..]) {
            Ok((expr, consumed)) => (expr, consumed),
            Err(e) => return Err(e),
        };

        Ok((
            Expression::Unary {
                operator,
                operand: Box::new(right),
            },
            right_consumed + 1,
        ))
    } else {
        parse_primary(tokens)
    }
}

fn parse_multiplicative(tokens: &[Token]) -> Result<(Expression, usize), String> {
    if tokens.is_empty() {
        return Err(
            "Unexpected end of input while parsing logical multiplicative expression.".to_string(),
        );
    }

    let (mut left, mut consumed) = match parse_unary(tokens) {
        Ok((expr, consumed)) => (expr, consumed),
        Err(e) => return Err(e),
    };

    while consumed < tokens.len() {
        if tokens.get(consumed) == Some(&Token::Div)
            || tokens.get(consumed) == Some(&Token::Mul)
            || tokens.get(consumed) == Some(&Token::Mod)
        {
            if consumed + 1 >= tokens.len() {
                return Err("Expected expression after operator.".to_string());
            }
            let operator = tokens[consumed].clone();
            let (right, right_consumed) = match parse_unary(&tokens[consumed + 1..]) {
                Ok((expr, consumed)) => (expr, consumed),
                Err(e) => return Err(e),
            };

            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
            consumed += right_consumed + 1;
        } else {
            break;
        }
    }
    Ok((left, consumed))
}

fn parse_additive(tokens: &[Token]) -> Result<(Expression, usize), String> {
    if tokens.is_empty() {
        return Err(
            "Unexpected end of input while parsing logical additive expression.".to_string(),
        );
    }

    let (mut left, mut consumed) = match parse_multiplicative(tokens) {
        Ok((expr, consumed)) => (expr, consumed),
        Err(e) => return Err(e),
    };

    while consumed < tokens.len() {
        if tokens.get(consumed) == Some(&Token::Add) || tokens.get(consumed) == Some(&Token::Sub) {
            if consumed + 1 >= tokens.len() {
                return Err("Expected expression after operator.".to_string());
            }
            let operator = tokens[consumed].clone();
            let (right, right_consumed) = match parse_multiplicative(&tokens[consumed + 1..]) {
                Ok((expr, consumed)) => (expr, consumed),
                Err(e) => return Err(e),
            };

            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
            consumed += right_consumed + 1;
        } else {
            break;
        }
    }
    Ok((left, consumed))
}

fn parse_relational(tokens: &[Token]) -> Result<(Expression, usize), String> {
    if tokens.is_empty() {
        return Err(
            "Unexpected end of input while parsing logical relational expression.".to_string(),
        );
    }

    let (mut left, mut consumed) = match parse_additive(tokens) {
        Ok((expr, consumed)) => (expr, consumed),
        Err(e) => return Err(e),
    };

    while consumed < tokens.len() {
        if tokens.get(consumed) == Some(&Token::Greater)
            || tokens.get(consumed) == Some(&Token::GreaterEqual)
            || tokens.get(consumed) == Some(&Token::Less)
            || tokens.get(consumed) == Some(&Token::LessEqual)
        {
            if consumed + 1 >= tokens.len() {
                return Err("Expected expression after operator.".to_string());
            }
            let operator = tokens[consumed].clone();
            let (right, right_consumed) = match parse_additive(&tokens[consumed + 1..]) {
                Ok((expr, consumed)) => (expr, consumed),
                Err(e) => return Err(e),
            };

            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
            consumed += right_consumed + 1;
        } else {
            break;
        }
    }
    Ok((left, consumed))
}
fn parse_logical_equality(tokens: &[Token]) -> Result<(Expression, usize), String> {
    if tokens.is_empty() {
        return Err(
            "Unexpected end of input while parsing logical equality expression.".to_string(),
        );
    }

    let (mut left, mut consumed) = match parse_relational(tokens) {
        Ok((expr, consumed)) => (expr, consumed),
        Err(e) => return Err(e),
    };

    while consumed < tokens.len() {
        if tokens.get(consumed) == Some(&Token::EqualEqual)
            || tokens.get(consumed) == Some(&Token::NotEqual)
        {
            if consumed + 1 >= tokens.len() {
                return Err("Expected expression after operator.".to_string());
            }
            let operator = tokens[consumed].clone();
            let (right, right_consumed) = match parse_relational(&tokens[consumed + 1..]) {
                Ok((expr, consumed)) => (expr, consumed),
                Err(e) => return Err(e),
            };

            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
            consumed += right_consumed + 1;
        } else {
            break;
        }
    }
    Ok((left, consumed))
}
fn parse_logical_and(tokens: &[Token]) -> Result<(Expression, usize), String> {
    if tokens.is_empty() {
        return Err("Unexpected end of input while parsing logical AND expression.".to_string());
    }

    let (mut left, mut consumed) = match parse_logical_equality(tokens) {
        Ok((expr, consumed)) => (expr, consumed),
        Err(e) => return Err(e),
    };

    while consumed < tokens.len() {
        if tokens.get(consumed) == Some(&Token::And) {
            if consumed + 1 >= tokens.len() {
                return Err("Expected expression after operator.".to_string());
            }
            let operator = tokens[consumed].clone();
            let (right, right_consumed) = match parse_logical_equality(&tokens[consumed + 1..]) {
                Ok((expr, consumed)) => (expr, consumed),
                Err(e) => return Err(e),
            };

            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
            consumed += right_consumed + 1;
        } else {
            break;
        }
    }
    Ok((left, consumed))
}
fn parse_logical_or(tokens: &[Token]) -> Result<(Expression, usize), String> {
    if tokens.is_empty() {
        return Err("Unexpected end of input while parsing logical OR expression.".to_string());
    }

    let (mut left, mut consumed) = match parse_logical_and(tokens) {
        Ok((expr, consumed)) => (expr, consumed),
        Err(e) => return Err(e),
    };

    while consumed < tokens.len() {
        if tokens.get(consumed) == Some(&Token::Or) {
            if consumed + 1 >= tokens.len() {
                return Err("Expected expression after operator".to_string());
            }
            let operator = tokens[consumed].clone();

            let (right, right_consumed) = match parse_logical_and(&tokens[consumed + 1..]) {
                Ok((expr, consumed)) => (expr, consumed),
                Err(e) => return Err(e),
            };

            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
            consumed += right_consumed + 1;
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
        let (expr, consumed) = match parse_logical_or(tokens) {
            Ok((expr, consumed)) => (expr, consumed),
            Err(e) => return Err(e),
        };

        if consumed != tokens.len() {
            return Err(format!("Unexpected token: {:?}", tokens[consumed]));
        }

        Ok((expr, consumed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    // ----- Primary Expression Tests -----
    #[test]
    fn test_parse_primary_identifier() {
        let tokens = vec![Token::Identifier("x".to_string())];
        let expected = Expression::Token(Token::Identifier("x".to_string()));
        assert_eq!(parse_primary(&tokens), Ok((expected, 1)));
    }

    #[test]
    fn test_parse_primary_number() {
        let tokens = vec![Token::Number(42)];
        let expected = Expression::Token(Token::Number(42));
        assert_eq!(parse_primary(&tokens), Ok((expected, 1)));
    }

    #[test]
    fn test_parse_primary_grouping() {
        // Test a grouped expression: ( x ).
        // In this simple case, that returns the identifier and consumes 1 token.
        // Then the grouped expression consumes 1 (LeftParen) + 1 (inner expr) + 1 (RightParen) = 3 tokens.

        let tokens = vec![
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::RightParen,
        ];
        let expected = Expression::Grouping(Box::new(Expression::Token(Token::Identifier(
            "x".to_string(),
        ))));

        assert_eq!(parse_primary(&tokens), Ok((expected, 3)));
    }

    // ----- Unary Expression Tests -----
    #[test]
    fn test_parse_unary_not() {
        let tokens = vec![Token::Not, Token::Identifier("x".to_string())];
        let expected = Expression::Unary {
            operator: Token::Not,
            operand: Box::new(Expression::Token(Token::Identifier("x".to_string()))),
        };
        assert_eq!(parse_unary(&tokens), Ok((expected, 2)));
    }

    // ----- Multiplicative Expression Tests -----
    #[test]
    fn test_parse_multiplicative_mul() {
        let tokens = vec![
            Token::Identifier("x".to_string()),
            Token::Mul,
            Token::Identifier("y".to_string()),
        ];
        let expected = Expression::Binary {
            left: Box::new(Expression::Token(Token::Identifier("x".to_string()))),
            operator: Token::Mul,
            right: Box::new(Expression::Token(Token::Identifier("y".to_string()))),
        };
        assert_eq!(parse_multiplicative(&tokens), Ok((expected, 3)));
    }

    #[test]
    fn test_parse_multiplicative_div() {
        let tokens = vec![
            Token::Identifier("x".to_string()),
            Token::Div,
            Token::Identifier("y".to_string()),
        ];
        let expected = Expression::Binary {
            left: Box::new(Expression::Token(Token::Identifier("x".to_string()))),
            operator: Token::Div,
            right: Box::new(Expression::Token(Token::Identifier("y".to_string()))),
        };
        assert_eq!(parse_multiplicative(&tokens), Ok((expected, 3)));
    }

    #[test]
    fn test_parse_multiplicative_mod() {
        let tokens = vec![
            Token::Identifier("x".to_string()),
            Token::Mod,
            Token::Identifier("y".to_string()),
        ];
        let expected = Expression::Binary {
            left: Box::new(Expression::Token(Token::Identifier("x".to_string()))),
            operator: Token::Mod,
            right: Box::new(Expression::Token(Token::Identifier("y".to_string()))),
        };
        assert_eq!(parse_multiplicative(&tokens), Ok((expected, 3)));
    }

    // ----- Additive Expression Tests -----
    #[test]
    fn test_parse_additive_chained() {
        // Test: x + y - z, which should be parsed as ((x + y) - z)
        let tokens = vec![
            Token::Identifier("x".to_string()),
            Token::Add,
            Token::Identifier("y".to_string()),
            Token::Sub,
            Token::Identifier("z".to_string()),
        ];
        let intermediate = Expression::Binary {
            left: Box::new(Expression::Token(Token::Identifier("x".to_string()))),
            operator: Token::Add,
            right: Box::new(Expression::Token(Token::Identifier("y".to_string()))),
        };
        let expected = Expression::Binary {
            left: Box::new(intermediate),
            operator: Token::Sub,
            right: Box::new(Expression::Token(Token::Identifier("z".to_string()))),
        };
        assert_eq!(parse_additive(&tokens), Ok((expected, 5)));
    }

    // ----- Relational Expression Tests -----
    #[test]
    fn test_parse_relational() {
        // Test: x < y
        let tokens = vec![
            Token::Identifier("x".to_string()),
            Token::Less,
            Token::Identifier("y".to_string()),
        ];
        let expected = Expression::Binary {
            left: Box::new(Expression::Token(Token::Identifier("x".to_string()))),
            operator: Token::Less,
            right: Box::new(Expression::Token(Token::Identifier("y".to_string()))),
        };
        assert_eq!(parse_relational(&tokens), Ok((expected, 3)));
    }

    // ----- Logical Equality Expression Tests -----
    #[test]
    fn test_parse_logical_equality() {
        // Test: x == y != z  which should be parsed as ((x == y) != z)
        let tokens = vec![
            Token::Identifier("x".to_string()),
            Token::EqualEqual,
            Token::Identifier("y".to_string()),
            Token::NotEqual,
            Token::Identifier("z".to_string()),
        ];
        let left = Expression::Binary {
            left: Box::new(Expression::Token(Token::Identifier("x".to_string()))),
            operator: Token::EqualEqual,
            right: Box::new(Expression::Token(Token::Identifier("y".to_string()))),
        };
        let expected = Expression::Binary {
            left: Box::new(left),
            operator: Token::NotEqual,
            right: Box::new(Expression::Token(Token::Identifier("z".to_string()))),
        };
        assert_eq!(parse_logical_equality(&tokens), Ok((expected, 5)));
    }

    // ----- Logical AND Expression Tests -----
    #[test]
    fn test_parse_logical_and() {
        // Test: x && y && z should be parsed as ((x && y) && z)
        let tokens = vec![
            Token::Identifier("x".to_string()),
            Token::And,
            Token::Identifier("y".to_string()),
            Token::And,
            Token::Identifier("z".to_string()),
        ];
        let left = Expression::Binary {
            left: Box::new(Expression::Token(Token::Identifier("x".to_string()))),
            operator: Token::And,
            right: Box::new(Expression::Token(Token::Identifier("y".to_string()))),
        };
        let expected = Expression::Binary {
            left: Box::new(left),
            operator: Token::And,
            right: Box::new(Expression::Token(Token::Identifier("z".to_string()))),
        };
        assert_eq!(parse_logical_and(&tokens), Ok((expected, 5)));
    }

    // ----- Logical OR Expression Tests -----
    #[test]
    fn test_parse_logical_or() {
        // Test: x || y || z should be parsed as ((x || y) || z)
        let tokens = vec![
            Token::Identifier("x".to_string()),
            Token::Or,
            Token::Identifier("y".to_string()),
            Token::Or,
            Token::Identifier("z".to_string()),
        ];
        let left = Expression::Binary {
            left: Box::new(Expression::Token(Token::Identifier("x".to_string()))),
            operator: Token::Or,
            right: Box::new(Expression::Token(Token::Identifier("y".to_string()))),
        };
        let expected = Expression::Binary {
            left: Box::new(left),
            operator: Token::Or,
            right: Box::new(Expression::Token(Token::Identifier("z".to_string()))),
        };
        assert_eq!(parse_logical_or(&tokens), Ok((expected, 5)));
    }

    // ----- Complete Expression Tests -----
    #[test]
    fn test_expression_new_complete() {
        // expression: x + y * z
        let tokens = vec![
            Token::Identifier("x".to_string()),
            Token::Add,
            Token::Identifier("y".to_string()),
            Token::Mul,
            Token::Identifier("z".to_string()),
        ];
        let mul = Expression::Binary {
            left: Box::new(Expression::Token(Token::Identifier("y".to_string()))),
            operator: Token::Mul,
            right: Box::new(Expression::Token(Token::Identifier("z".to_string()))),
        };
        let expected = Expression::Binary {
            left: Box::new(Expression::Token(Token::Identifier("x".to_string()))),
            operator: Token::Add,
            right: Box::new(mul),
        };
        assert_eq!(Expression::new(&tokens), Ok((expected, tokens.len())));
    }

    #[test]
    fn test_expression_new_extra_tokens() {
        // Test error when extra tokens remain: "x y"
        let tokens = vec![
            Token::Identifier("x".to_string()),
            Token::Identifier("y".to_string()),
        ];
        let result = Expression::new(&tokens);
        assert!(result.is_err());
        if let Err(msg) = result {
            assert!(msg.contains("Unexpected token"));
        }
    }

    #[test]
    fn test_expression_new_trailing_operator() {
        // Test error for a trailing operator: "x &&" (missing right-hand operand)
        let tokens = vec![Token::Identifier("x".to_string()), Token::And];
        let result = Expression::new(&tokens);
        assert!(result.is_err());
        if let Err(msg) = result {
            assert!(msg.contains("Expected expression after operator"));
        }
    }
    // ----- Function Call Tests -----
    #[test]
    fn test_parse_function_call_no_args() {
        // Test: foo()
        let tokens = vec![
            Token::Identifier("foo".to_string()),
            Token::LeftParen,
            Token::RightParen,
        ];
        let expected = Expression::FunctionCall {
            identifier: Token::Identifier("foo".to_string()),
            args: vec![],
        };
        assert_eq!(parse_function_call(&tokens), Ok((expected, 3)));
    }

    #[test]
    fn test_parse_function_call_single_arg() {
        // Test: foo(x)
        let tokens = vec![
            Token::Identifier("foo".to_string()),
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::RightParen,
        ];
        let expected = Expression::FunctionCall {
            identifier: Token::Identifier("foo".to_string()),
            args: vec![Token::Identifier("x".to_string())],
        };
        assert_eq!(parse_function_call(&tokens), Ok((expected, 4)));
    }

    #[test]
    fn test_parse_function_call_multiple_args() {
        // Test: foo(x, y, z)
        let tokens = vec![
            Token::Identifier("foo".to_string()),
            Token::LeftParen,
            Token::Identifier("x".to_string()),
            Token::Comma,
            Token::Identifier("y".to_string()),
            Token::Comma,
            Token::Identifier("z".to_string()),
            Token::RightParen,
        ];
        let expected = Expression::FunctionCall {
            identifier: Token::Identifier("foo".to_string()),
            args: vec![
                Token::Identifier("x".to_string()),
                Token::Identifier("y".to_string()),
                Token::Identifier("z".to_string()),
            ],
        };
        assert_eq!(parse_function_call(&tokens), Ok((expected, 8)));
    }
}
