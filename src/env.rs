use std::collections::HashMap;

use crate::types::Type;

#[derive(Debug, PartialEq, Default)]
pub(crate) struct Env {
    bindings: HashMap<String, Type>,
}

impl Env {
    pub(crate) fn store_binding(&mut self, name: String, val: Type) {
        self.bindings.insert(name, val);
    }
}
