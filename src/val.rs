#[derive(Debug, PartialEq, Clone)]
pub enum Val {
    Number(i32),
    Boolean(bool),
    Literal(String),
    String(String),
}

impl Val {
    pub fn parse_boolean(s: &str) -> Result<(bool, &str), String> {
        if s.starts_with("true") {
            Ok((true, &s[4..]))
        } else if s.starts_with("false") {
            Ok((false, &s[5..]))
        } else {
            Err(format!("Illegal Boolean Value: {}", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::val::Val;

    #[test]
    fn parse_boolean_true() {
        assert_eq!(Val::parse_boolean("true"), Ok((true, "")));
    }
}
