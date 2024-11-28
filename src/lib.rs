use utils::extract_next_digit;

mod env;
mod utils;

pub mod binding;
pub mod delimeter;
pub mod expression;
pub mod operator;
pub mod token;
pub mod val;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);
impl Number {
    pub fn new(s: &str) -> (Self, &str) {
        let (number, rest) = extract_next_digit(s.trim());
        (Self(number.parse().unwrap()), rest)
    }
}
