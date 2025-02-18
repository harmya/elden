use function::Function;
use token::Token;
pub mod binding;
mod env;
pub mod expression;
mod function;
pub mod statement;
pub mod token;
mod utils;

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

    let func = Function::new(&tokens);
    match func {
        Ok(f) => println!("\n{:?} \n", f),
        Err(e) => eprintln!("Failed to parse function: {}", e),
    }
    Ok((tokens, main_index))
}
