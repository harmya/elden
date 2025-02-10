use delimeter::Delimiter;
use token::Token;

mod env;
mod utils;

pub mod binding;
pub mod delimeter;
pub mod expression;
pub mod keyword;
pub mod operator;
pub mod token;
pub mod types;

pub fn run_lexer(input: &str) -> Result<(Vec<Token>, &str), String> {
    let mut remaining = input.trim();
    let mut tokens = Vec::new();

    while !remaining.is_empty() {
        match Token::new(remaining) {
            Ok((token, rest)) if token == Token::Delimiter(Delimiter::DoubleQuote) => {
                let (value, rest) = match Delimiter::process_literal(rest) {
                    Ok(result) => result,
                    Err(err) => return Err(err),
                };
                tokens.push(Token::Type(types::Type::String(value.to_string())));

                remaining = rest.trim();
            }
            Ok((token, rest)) => {
                tokens.push(token);
                remaining = rest.trim();
            }
            Err(err) => return Err(err),
        }
    }

    Ok((tokens, remaining))
}
