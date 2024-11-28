use crate::{delimeter::Delimeter, keyword::Keyword, operator::Operator, types::Type};

#[derive(Debug, PartialEq)]
pub enum Token {
    Delimter(Delimeter),
    Operator(Operator),
    Literal(Type),
    Keyword(Keyword),
}
