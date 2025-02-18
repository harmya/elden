use crate::{env::Env, expression::Expression, token::Token};

#[derive(Debug, PartialEq)]
pub struct BindingDef {
    pub name: Token,
    pub val: Expression,
}

impl BindingDef {
    pub fn new(s: &str) -> Result<(Self, &str), String> {
        todo!()
    }

    pub(crate) fn eval(&self, env: &mut Env) {
        todo!()
    }
}
