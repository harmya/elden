use crate::{
    operator::{self, Operator},
    types::Type,
    utils::{extract_next_ident, extract_operator_and_delimiter, extract_whitespace},
};

pub struct Binary {
    pub left_operand: Type,
    pub operator: Operator,
    pub right_operand: Type,
}
impl Binary {
    pub fn new(s: &str) -> Result<(Self, &str), String> {
        let (left_operand, rest) = match Type::new(s) {
            Ok(res) => res,
            Err(e) => {
                return Err(format!("{}", e));
            }
        };

        let (_, rest) = extract_whitespace(rest);

        let (operator, _) = match Operator::new(rest) {
            Ok(res) => res,
            Err(e) => {
                return Err(format!("{}", e));
            }
        };

        let (_, rest) = extract_whitespace(rest);
        let (right_operand, rest) = match Type::new(rest) {
            Ok(res) => res,
            Err(e) => {
                return Err(format!("{}", e));
            }
        };

        let binary = Binary {
            left_operand,
            operator,
            right_operand,
        };
        return Ok((binary, rest));
    }
}
