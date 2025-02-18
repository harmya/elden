use token::Token;

mod env;

pub mod binding;
pub mod expression;
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
