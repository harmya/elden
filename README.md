### Writing a compiler from scratch

Parts of a compiler: 
- Lexer
- Parser/Generate AST
- Semantic Analysis
- IR Generation
- Optimization Passes
- Code Generation

I might skip the backend and make this an interpreter (easier :D )

Updates:

**Lexer:**
Consider a program as a string, loop through it token by token, and assign each token.
Each Token can be: 
``` rust
pub enum Token { Comma, LeftParen, RightParen, LeftBrace, RightBrace, SemiColon, DoubleQuote, Add, Sub, Mul, Div, Mod, NotEqual, EqualEqual, Greater, GreaterEqual, Less, LessEqual, Equal, Not, Or, And, Number(i32), Boolean(bool), Identifier(String), StringLiteral(String), Func, Main, If, Else, For, While, Let, Return, Print }
```

So for a given program like this:
```rust
func check(x, y) {
    let sum = x + y;
    let more_than_20 = sum >= 10;
    let less_than_30 = sum < 30;
    return more_than_20 && less_than_30;
}

func main() {
    let x = 10;
    let y = 20;
    return check(x, y);
}
```

Token stream: Func, Identifier("check"), LeftParen, Identifier("x"), Comma, Identifier("y"), RightParen, LeftBrace, Let, Identifier("sum"), Equal, Identifier("x"), Add, Identifier("y"), SemiColon, Let, Identifier("more_than_20"), Equal, Identifier("sum"), GreaterEqual, Number(10), SemiColon, Let, Identifier("less_than_30"), Equal, Identifier("sum"), Less, Number(30), SemiColon, Return, Identifier("more_than_20"), And, Identifier("less_than_30"), SemiColon, RightBrace, Func, Main, LeftParen, RightParen, LeftBrace, Let, Identifier("x"), Equal, Number(10), SemiColon, Let, Identifier("y"), Equal, Number(20), SemiColon, Return, Identifier("check"), LeftParen, Identifier("x"), Comma, Identifier("y"), RightParen, SemiColon, RightBrace

**Parser/Generate AST:**
Each program is a vector of functions. Each function is a vector of statements. And each statements is a slight variation of using keywords and statements.
Structure of a program:
```rust
struct Program {
    pub functions: Vec<Function>,
}
```
Structure of a function:
```rust
struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Statement>,
}
```
Types of a statement:
```rust
enum Statement {
    AssignStatement {
        identifier: Token,
        value: Expression,
    },
    IfStatement // to-do,
    WhileStatement //to-do,
    ReturnStatement {
        value: Expression,
    },
}
```

Types of an expression:
```rust
enum Expression {
    FunctionCall {
        identifier: Token,
        args: Vec<Token>,
    },
    Token(Token),
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Unary {
        operator: Token,
        operand: Box<Expression>,
    },
    Grouping(Box<Expression>),
}
```

Now, we use normal precedence to organize the structure of this program. Parse each function.
So for a given program like this:
```rust
func check(x, y) {
    let sum = x + y;
    let more_than_20 = sum >= 10;
    let less_than_30 = sum < 30;
    return more_than_20 && less_than_30;
}

func main() {
    let x = 10;
    let y = 20;
    return check(x, y);
}
```
```
The AST is:
Function: Identifier("check")
├── Parameters:
│   ├── Identifier("x")
│   ├── Identifier("y")
└── Body:
│   ├── AssignStatement: Identifier("sum")
│   │   ├── Value:
│   │   │   ├── Operator: Add
│   │   │   ├── Left:
│   │   │   │   ├── Identifier("x")
│   │   │   ├── Right:
│   │   │   │   ├── Identifier("y")
│   ├── AssignStatement: Identifier("more_than_20")
│   │   ├── Value:
│   │   │   ├── Operator: GreaterEqual
│   │   │   ├── Left:
│   │   │   │   ├── Identifier("sum")
│   │   │   ├── Right:
│   │   │   │   ├── Number(10)
│   ├── AssignStatement: Identifier("less_than_30")
│   │   ├── Value:
│   │   │   ├── Operator: Less
│   │   │   ├── Left:
│   │   │   │   ├── Identifier("sum")
│   │   │   ├── Right:
│   │   │   │   ├── Number(30)
│   ├── ReturnStatement
│   │   ├── Value:
│   │   │   ├── Operator: And
│   │   │   ├── Left:
│   │   │   │   ├── Identifier("more_than_20")
│   │   │   ├── Right:
│   │   │   │   ├── Identifier("less_than_30")
Function: Main
├── Parameters:
└── Body:
│   ├── AssignStatement: Identifier("x")
│   │   ├── Value:
│   │   │   ├── Number(10)
│   ├── AssignStatement: Identifier("y")
│   │   ├── Value:
│   │   │   ├── Number(20)
│   ├── ReturnStatement
│   │   ├── Value:
│   │   │   ├── Function Call:
│   │   │   │   ├── Identifier: Identifier("check")
│   │   │   │   ├── Arguments:
│   │   │   │   │   ├── Identifier("x")
│   │   │   │   │   ├── Identifier("y")
```







