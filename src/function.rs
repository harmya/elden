#[derive(Debug)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Statement>,
}

impl Function {
    pub fn from_tokens(tokens: &[Token]) -> Result<Function, ParseError> {
        if tokens.is_empty() || tokens[0] !=  {
            return Err(ParseError::new("Expected 'fn' keyword"));
        }
    }
}
