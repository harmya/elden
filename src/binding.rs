use crate::{
    env::Env,
    expression::Expression,
    utils::{extract_next_ident, extract_whitespace, tag},
};

#[derive(Debug, PartialEq)]
pub struct BindingDef {
    pub name: String,
    pub val: Expression,
}

impl BindingDef {
    pub fn new(s: &str) -> Result<(Self, &str), String> {
        let def = match tag("let ", s) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        let (_, def) = extract_whitespace(def);
        let (name, rest) = extract_next_ident(def);

        let rest = match tag("=", rest) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        let (_, rest) = extract_whitespace(rest);
        todo!();
    }

    pub(crate) fn eval(&self, env: &mut Env) {
        todo!()
    }
}
