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

        let (val, rest) = match Expression::new(rest) {
            Ok(res) => res,
            Err(err) => return Err(err),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::Expression;

    fn parse_expression(input: &str) -> Expression {
        let (expression, _) = Expression::new(input).unwrap();
        return expression;
    }

    #[test]
    fn test_binding_def_new_valid_input() {
        let input = "let x = 42";
        let (binding, rest) = BindingDef::new(input).expect("Failed to parse binding definition");

        assert_eq!(binding.name, "x");
        assert_eq!(binding.val, parse_expression("42"));
        assert_eq!(rest, "");
    }

    #[test]
    fn test_binding_def_new_invalid_keyword() {
        let input = "letx = 42";
        let result = BindingDef::new(input);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "expected let ");
    }

    #[test]
    fn test_binding_def_new_missing_equal_sign() {
        let input = "let x 42"; // Missing '='
        let result = BindingDef::new(input);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "expected =");
    }

    #[test]
    fn test_binding_def_new_invalid_expression() {
        let input = "let x = ";
        let result = BindingDef::new(input);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid expression: ''");
    }
}
