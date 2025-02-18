use std::collections::HashMap;

use crate::{expression::Expression, token::Token};

#[derive(Debug, PartialEq, Default)]
pub(crate) struct Env {
    bindings: HashMap<Token, Expression>,
}

impl Env {
    pub(crate) fn store_binding(&mut self, name: Token, val: Expression) {
        self.bindings.insert(name, val);
    }
}
