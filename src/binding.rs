use std::fmt::format;

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
        let def = tag("let", s);
        let (_, def) = extract_whitespace(def);
        let (name, rest) = extract_next_ident(def);

        let rest = tag("=", rest);
        let (_, rest) = extract_whitespace(rest);

        let (val, rest) = match Expression::new(rest) {
            Ok(res) => res,
            Err(err) => return Err(format!("{}", err)),
        };

        Ok((
            Self {
                name: name.to_string(),
                val,
            },
            rest,
        ))
    }

    pub(crate) fn eval(&self, env: &mut Env) {
        env.store_binding(self.name.clone(), self.val.eval());
    }
}
