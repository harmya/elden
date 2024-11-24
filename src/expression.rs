use crate::{operator::Operator, utils::extract_whitespace, Number};

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub first_operand: Number,
    pub second_operand: Number,
    pub operator: Operator,
}

impl Expression {
    pub fn new(s: &str) -> (Self, &str) {
        let (first_operand, rest) = Number::new(s.trim());
        let (_, rest) = extract_whitespace(rest);

        let (operator, rest) = Operator::new(rest.trim());
        let (_, rest) = extract_whitespace(rest);

        let (second_operand, rest) = Number::new(rest.trim());
        (
            Self {
                first_operand,
                second_operand,
                operator,
            },
            rest,
        )
    }
}
