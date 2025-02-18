use crate::{function::Function, token::Token};

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}

impl Program {
    pub fn new(tokens: &Vec<Token>) -> Result<Self, String> {
        let mut functions = Vec::new();
        let mut index = 0;

        while index < tokens.len() {
            match Function::new(tokens, index) {
                Ok((func, consumed)) => {
                    functions.push(func);
                    index += consumed;
                }
                Err(err) => {
                    return Err(format!(
                        "Error parsing function at index {}: {}",
                        index, err
                    ))
                }
            }
        }

        Ok(Program { functions })
    }
}
