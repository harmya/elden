use function::print_function;
use program::Program;
use token::Token;
pub mod expression;
mod function;
pub mod program;
pub mod statement;
pub mod symbol;
pub mod token;

pub fn run_lexer(input: &str) -> Result<(Vec<Token>, usize), String> {
    let mut remaining = input.trim();
    let mut tokens = Vec::new();
    let mut main_index = 0;
    let mut curr_index = 0;

    while !remaining.is_empty() {
        match Token::new(remaining) {
            Ok((token, rest)) => {
                // If the token is `Main`, record its index.
                if token == Token::Main {
                    main_index = curr_index;
                }
                tokens.push(token);
                remaining = rest.trim();
            }
            Err(err) => return Err(err),
        }
        curr_index += 1;
    }

    Ok((tokens, main_index))
}

pub fn get_program(input: &Vec<Token>) -> Result<Program, String> {
    Program::new(input)
}

pub fn print_ast(program: &Program) {
    for function in &program.functions {
        print_function(function);
    }
}
