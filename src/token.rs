#[derive(Debug, PartialEq)]
pub enum TokenType {
    ArithmeticOperator,
    RelationalOperator,
    LogicalOperator,
    AssignmentOperator,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
}
