### Writing a compiler from scratch

Parts of a compiler: 
- Lexer
- Parser
- Semantic Analysis
- IR Generation
- Optimization Passes
- Code Generation

I might skip the backend and make this an interpreter (easier?)

Current progress:

Lexer: Done
Consider a program as a long string, loop through it token by token, and assign each token.
Each Token can be: 
``` rust
pub enum Token {
    // Delimiters
    Comma,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    SemiColon,
    DoubleQuote,
    // Operators
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    NotEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    Not,
    Or,
    And,
    // Literals
    Number(i32),
    Boolean(bool),
    Identifier(String),
    StringLiteral(String),
    // Keywords
    Func,
    Main,
    If,
    Else,
    For,
    While,
    Let,
    Return,
    Print,
}
```
