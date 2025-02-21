use crate::expression::Expression;
use crate::statement::Statement;
use crate::token::Token;

#[derive(Debug)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Statement>,
}

fn print_expression(expr: &Expression, indent: usize) {
    let prefix = "│   ".repeat(indent);

    match expr {
        Expression::Token(token) => {
            println!("{}├── {:?}", prefix, token);
        }
        Expression::Binary {
            left,
            operator,
            right,
        } => {
            println!("{}├── Operator: {:?}", prefix, operator);
            println!("{}├── Left:", prefix);
            print_expression(left, indent + 1);
            println!("{}├── Right:", prefix);
            print_expression(right, indent + 1);
        }
        Expression::Unary { operator, operand } => {
            println!("{}├── Unary:", prefix);
            println!("{}│   ├── Operator: {:?}", prefix, operator);
            println!("{}│   ├── Operand:", prefix);
            print_expression(operand, indent + 2);
        }
        Expression::Grouping(inner) => {
            println!("{}├── Grouping:", prefix);
            print_expression(inner, indent + 1);
        }
        Expression::FunctionCall { identifier, args } => {
            println!("{}├── Function Call:", prefix);
            println!("{}│   ├── Identifier: {:?}", prefix, identifier);
            println!("{}│   ├── Arguments:", prefix);
            for arg in args {
                println!("{}│   │   ├── {:?}", prefix, arg);
            }
        }
        Expression::ArrayDec { arr_expr } => {
            println!("{}│   ├── Value: {:?}", prefix, arr_expr);
        }
        Expression::AccessIndex { ident, index } => {
            println!("{}├── Access Index:", prefix);
            println!("{}│   ├── Identifier: {:?} ", prefix, ident);
            println!("{}│   ├── Index:", prefix);
            print_expression(index, indent + 2);
        }
        Expression::GetLength { ident } => {
            println!("{}├── Get Length:", prefix);
            println!("{}│   ├── Identifier: {:?}", prefix, ident);
        }
        Expression::ArrayAppend { array, value } => {
            println!("{}├── Array Append:", prefix);
            println!("{}│   ├── Array: {:?} ", prefix, array);
            println!("{}│   ├── Value:", prefix);
            print_expression(value, indent + 2);
        }
    }
}

fn print_statement(stmt: &Statement, indent: usize) {
    let prefix = "│   ".repeat(indent);

    match stmt {
        Statement::AssignStatement { identifier, value } => {
            println!("{}├── AssignStatement: {:?}", prefix, identifier);
            println!("{}│   ├── Value:", prefix);
            print_expression(value, indent + 2);
        }
        Statement::DeclareStatement { identifier, value } => {
            println!("{}├── DeclareStatement: {:?}", prefix, identifier);
            println!("{}│   ├── Value:", prefix);
            if let Some(expr) = value {
                print_expression(expr, indent + 2);
            }
        }
        Statement::ReturnStatement { value } => {
            println!("{}├── ReturnStatement", prefix);
            println!("{}│   ├── Value:", prefix);
            print_expression(value, indent + 2);
        }

        Statement::IfStatement {
            cond,
            if_then,
            else_then,
        } => {
            println!("{}├── IfStatement", prefix);
            println!("{}│   ├── Condition:", prefix);
            print_expression(cond, indent + 2);
            println!("{}│   ├── If Then:", prefix);
            for stmt in if_then {
                print_statement(stmt, indent + 2);
            }
            if let Some(else_stmts) = else_then {
                println!("{}│   ├── Else Then:", prefix);
                for stmt in else_stmts {
                    print_statement(stmt, indent + 2);
                }
            }
        }

        Statement::WhileStatement { cond, loop_stmt } => {
            println!("{}├── WhileStatement", prefix);
            println!("{}│   ├── Condition:", prefix);
            print_expression(cond, indent + 2);
            println!("{}│   ├── Loop Body:", prefix);
            for stmt in loop_stmt {
                print_statement(stmt, indent + 2);
            }
        }

        Statement::ArrayAppend { identifier, value } => {
            println!("{}├── ArrayAppend: {:?}", prefix, identifier);
            println!("{}│   ├── Value:", prefix);
            print_expression(value, indent + 2);
        }
    }
}

pub fn print_function(func: &Function) {
    println!("Function: {:?}", func.name);
    println!("├── Parameters:");
    for param in &func.params {
        println!("│   ├── {:?}", param);
    }
    println!("└── Body:");
    for stmt in &func.body {
        print_statement(stmt, 1);
    }
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

    pub fn new(tokens: &Vec<Token>, start: usize) -> Result<(Self, usize), String> {
        if tokens.is_empty() || start >= tokens.len() || tokens[start] != Token::Func {
            return Err("Expected a function starting with the 'func' token".into());
        }

        let mut new_function: Function = Function {
            name: Token::Identifier("".to_string()),
            params: vec![],
            body: vec![],
        };

        // Expect the function name to be an identifier
        if tokens.len() > start + 2 {
            if let Token::Identifier(_) = tokens[start + 1] {
                new_function.name = tokens[start + 1].clone();
            } else if let Token::Main = tokens[start + 1] {
                new_function.name = tokens[start + 1].clone();
            } else {
                return Err("Syntax error, expected an identifier for the function".into());
            }
        } else {
            return Err("Syntax error, incomplete function header".into());
        }

        // Expect a left parenthesis after the function name
        if tokens.len() > start + 3 && tokens[start + 2] != Token::LeftParen {
            return Err("Syntax error, expected a left parenthesis".into());
        }

        let mut curr_index = start + 2;
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
            // Convert the tokens into a Statement
            let (statement, consumed) = match Statement::new(&tokens[curr_index..]) {
                Ok(output) => output,
                Err(err) => return Err(err),
            };

            new_function.body.push(statement);
            // Consume the semicolon
            curr_index += consumed;
        }

        // After parsing the body, we expect a closing brace
        if curr_index >= tokens.len() || tokens[curr_index] != Token::RightBrace {
            return Err("Syntax error, expected a closing brace at end of function body".into());
        }
        // Consume the right brace.
        curr_index += 1;

        Ok((new_function, curr_index))
    }
}
