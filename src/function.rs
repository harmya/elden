use crate::statement::Statement;
use crate::token::Token;

#[derive(Debug)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Statement>,
}

impl Function {
    // Constructs a Function from a slice of tokens
    // Returns an error if the tokens do not start with a `func` token

    // This function expects tokens in the following order:
    // - `Func` token
    // - An identifier for the function name
    // - A left parenthesis, then zero or more parameter identifiers (separated by commas), then a right parenthesis
    // - A left brace
    // - A vector of statements, each ending with a semicolon
    // - A right brace

    pub fn new(tokens: &Vec<Token>) -> Result<(Self, usize), String> {
        if tokens.is_empty() || tokens[0] != Token::Func {
            return Err("Expected a function starting with the 'func' token".into());
        }

        let mut new_function: Function = Function {
            name: Token::Identifier("".to_string()),
            params: vec![],
            body: vec![],
        };

        // Expect the function name to be an identifier
        if tokens.len() > 2 {
            if let Token::Identifier(_) = tokens[1] {
                new_function.name = tokens[1].clone();
            } else {
                return Err("Syntax error, expected an identifier for the function".into());
            }
        } else {
            return Err("Syntax error, incomplete function header".into());
        }

        // Expect a left parenthesis after the function name
        if tokens.len() > 3 && tokens[2] != Token::LeftParen {
            return Err("Syntax error, expected a left parenthesis".into());
        }

        let mut curr_index = 2;
        let mut found_right_paren = false;

        // Parse the parameters
        while curr_index < tokens.len() {
            if tokens[curr_index] == Token::RightParen {
                found_right_paren = true;
                curr_index += 1;
                break;
            }

            // If we encounter a left brace before the closing parenthesis, it's an error
            if tokens[curr_index] == Token::LeftBrace && !found_right_paren {
                return Err("Syntax error, expected right parenthesis before '{'".into());
            }

            // If the token is an identifier, add it as a parameter
            if let Token::Identifier(_) = tokens[curr_index] {
                new_function.params.push(tokens[curr_index].clone());
            }

            // Check for a comma with no argument following
            if curr_index < tokens.len() - 1
                && tokens[curr_index] == Token::Comma
                && tokens[curr_index + 1] == Token::RightParen
            {
                return Err("Syntax error, expected an argument between commas".into());
            }
            curr_index += 1;
        }

        if !found_right_paren {
            return Err("Syntax error, expected a closing parenthesis for parameters".into());
        }

        // Expect a left brace to start the function body
        if curr_index < tokens.len() {
            if tokens[curr_index] != Token::LeftBrace {
                return Err("Syntax error, expected an opening brace for function body".into());
            } else {
                // Consume the left brace
                curr_index += 1;
            }
        } else {
            return Err("Syntax error, expected function body".into());
        }

        // --- Parse the function body ---
        while curr_index < tokens.len() && tokens[curr_index] != Token::RightBrace {
            // Collect tokens until a semicolon is found
            let start_index = curr_index;
            while curr_index < tokens.len() && tokens[curr_index] != Token::SemiColon {
                curr_index += 1;
            }

            // If we've reached the end without finding a semicolon, it's a syntax error
            if curr_index >= tokens.len() {
                return Err(
                    "Syntax error, expected semicolon at end of statement in function body".into(),
                );
            }

            // Slice containing tokens for the current statement
            let statement_tokens = &tokens[start_index..curr_index];

            // Convert the tokens into a Statement
            let statement = Statement::new(statement_tokens)?;
            new_function.body.push(statement.0);

            // Consume the semicolon
            curr_index += 1;

            break;
        }

        // // After parsing the body, we expect a closing brace
        // if curr_index >= tokens.len() || tokens[curr_index] != Token::RightBrace {
        //     return Err("Syntax error, expected a closing brace at end of function body".into());
        // }
        // // Consume the right brace.
        // curr_index += 1;

        Ok((new_function, curr_index))
    }
}
