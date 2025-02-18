use crate::token::Token;

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

fn parse_primary(tokens: &[Token]) -> Result<(Expression, usize), String> {
    if tokens.is_empty() {
        return Err("Unexpected end of input while parsing logical AND expression.".to_string());
    }

    match tokens.get(0) {
        Some(Token::Number(_)) | Some(Token::StringLiteral(_)) | Some(Token::Identifier(_)) => {
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
            Err(_) => return Err("Expected an expression".into()),
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
        Err(_) => return Err("Expected an expression".into()),
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
                Err(_) => return Err("Expected an expression".into()),
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
        Err(_) => return Err("Expected an expression".into()),
    };

    while consumed < tokens.len() {
        if tokens.get(consumed) == Some(&Token::Add) || tokens.get(consumed) == Some(&Token::Sub) {
            if consumed + 1 >= tokens.len() {
                return Err("Expected expression after operator.".to_string());
            }
            let operator = tokens[consumed].clone();
            let (right, right_consumed) = match parse_multiplicative(&tokens[consumed + 1..]) {
                Ok((expr, consumed)) => (expr, consumed),
                Err(_) => return Err("Expected an expression".into()),
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
        Err(_) => return Err("Expected an expression".into()),
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
                Err(_) => return Err("Expected an expression".into()),
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
        Err(_) => return Err("Expected an expression".into()),
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
                Err(_) => return Err("Expected an expression".into()),
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
        Err(_) => return Err("Expected an expression".into()),
    };

    while consumed < tokens.len() {
        if tokens.get(consumed) == Some(&Token::And) {
            if consumed + 1 >= tokens.len() {
                return Err("Expected expression after operator.".to_string());
            }
            let operator = tokens[consumed].clone();
            let (right, right_consumed) = match parse_logical_and(&tokens[consumed + 1..]) {
                Ok((expr, consumed)) => (expr, consumed),
                Err(_) => return Err("Expected an expression".into()),
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
    println!("tokens in Expr {:?} \n", tokens);

    let (mut left, mut consumed) = match parse_logical_and(tokens) {
        Ok((expr, consumed)) => (expr, consumed),
        Err(_) => return Err("Expected an expression".into()),
    };

    while consumed < tokens.len() {
        if tokens.get(consumed) == Some(&Token::Or) {
            if consumed + 1 >= tokens.len() {
                return Err("Expected expression after operator".to_string());
            }
            let operator = tokens[consumed].clone();

            let (right, right_consumed) = match parse_logical_and(&tokens[consumed + 1..]) {
                Ok((expr, consumed)) => (expr, consumed),
                Err(_) => return Err("Expected an expression".into()),
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
            Err(_) => return Err("Expected an expression".into()),
        };

        if consumed != tokens.len() {
            return Err(format!("Unexpected token: {:?}", tokens[consumed]));
        }

        Ok((expr, consumed))
    }
}
