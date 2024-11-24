use std::collections::HashMap;

use crate::val::Val;

#[derive(Debug, PartialEq, Default)]
pub(crate) struct Env {
    bindings: HashMap<String, Val>,
}
