### Writing a compiler from scratch (WIP)

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

```rust
pub enum Token { Comma, LeftParen, RightParen, LeftBrace, RightBrace, SemiColon, DoubleQuote, Add, Sub, Mul, Div, Mod, NotEqual, EqualEqual, Greater, GreaterEqual, Less, LessEqual, Equal, Not, Or, And, Number(i32), Boolean(bool), Identifier(String), StringLiteral(String), Func, Main, If, Else, For, While, Let, Return, Print }
```

So for a given program like this:

```rust
func sum_check(x, y) {
    let sum = x + y;
    let more_than_20 = sum >= 20;
    let less_than_30 = sum < 30;
    return more_than_20 && less_than_30;
}

func main() {
    let x = 10;
    let y = 20;
    return check(x, y);
}
```

Token stream:
`Func, Identifier("check"), LeftParen, Identifier("x"), Comma, Identifier("y"), RightParen, LeftBrace, Let, Identifier("sum"), Equal, Identifier("x"), Add, Identifier("y"), SemiColon, Let, Identifier("more_than_20"), Equal, Identifier("sum"), GreaterEqual, Number(10), SemiColon, Let, Identifier("less_than_30"), Equal, Identifier("sum"), Less, Number(30), SemiColon, Return, Identifier("more_than_20"), And, Identifier("less_than_30"), SemiColon, RightBrace, Func, Main, LeftParen, RightParen, LeftBrace, Let, Identifier("x"), Equal, Number(10), SemiColon, Let, Identifier("y"), Equal, Number(20), SemiColon, Return, Identifier("check"), LeftParen, Identifier("x"), Comma, Identifier("y"), RightParen, SemiColon, RightBrace`

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

Now, we use normal precedence to organize the structure of this program. Take each token at a time, and recursively call it on the rest of the tokens based on conditions for the token. Example flow: Consider if the current token is "let", then the next token has to be an identifier (otherwise throw an error), which should be followed an "Equal" operator, then there should an expression. I will probably spend a few annoying minutes later to formally write the down the grammer, but you get the idea.

So for a given program like this:

```go
func sum_check(x, y) {
    let sum = x + y;
    let more_than_20 = sum >= 20;
    let less_than_30 = sum < 30;
    return more_than_20 && less_than_30;
}

func main() {
    let x = 10;
    let y = 20;
    return check(x, y);
}
```

The AST is:

```
Function: Identifier("sum_check")
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
│   │   │   │   ├── Number(20)
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

We can also have if else statements, in the program:

```go
func main() {
    let x = 2;
    let y = 4;
    if (x > y) {
        let sum = x + y;
        return sum;
    } else if (x < y) {
        let diff = x - y;
        return diff;
    } else {
        return 0;
    }
}
```

The AST is:

```Function: Main
├── Parameters:
└── Body:
│   ├── AssignStatement: Identifier("x")
│   │   ├── Value:
│   │   │   ├── Number(2)
│   ├── AssignStatement: Identifier("y")
│   │   ├── Value:
│   │   │   ├── Number(4)
│   ├── IfStatement
│   │   ├── Condition:
│   │   │   ├── Operator: Greater
│   │   │   ├── Left:
│   │   │   │   ├── Identifier("x")
│   │   │   ├── Right:
│   │   │   │   ├── Identifier("y")
│   │   ├── If Then:
│   │   │   ├── AssignStatement: Identifier("sum")
│   │   │   │   ├── Value:
│   │   │   │   │   ├── Operator: Add
│   │   │   │   │   ├── Left:
│   │   │   │   │   │   ├── Identifier("x")
│   │   │   │   │   ├── Right:
│   │   │   │   │   │   ├── Identifier("y")
│   │   │   ├── ReturnStatement
│   │   │   │   ├── Value:
│   │   │   │   │   ├── Identifier("sum")
│   │   ├── Else Then:
│   │   │   ├── IfStatement
│   │   │   │   ├── Condition:
│   │   │   │   │   ├── Operator: Less
│   │   │   │   │   ├── Left:
│   │   │   │   │   │   ├── Identifier("x")
│   │   │   │   │   ├── Right:
│   │   │   │   │   │   ├── Identifier("y")
│   │   │   │   ├── If Then:
│   │   │   │   │   ├── AssignStatement: Identifier("diff")
│   │   │   │   │   │   ├── Value:
│   │   │   │   │   │   │   ├── Operator: Sub
│   │   │   │   │   │   │   ├── Left:
│   │   │   │   │   │   │   │   ├── Identifier("x")
│   │   │   │   │   │   │   ├── Right:
│   │   │   │   │   │   │   │   ├── Identifier("y")
│   │   │   │   │   ├── ReturnStatement
│   │   │   │   │   │   ├── Value:
│   │   │   │   │   │   │   ├── Identifier("diff")
│   │   │   │   ├── Else Then:
│   │   │   │   │   ├── ReturnStatement
│   │   │   │   │   │   ├── Value:
│   │   │   │   │   │   │   ├── Number(0)
```

We can also have while loops, in the program:

```go
func main() {
    let i = 0;
    let sum = 0;
    while (i < 10) {
        sum = sum + i;
        i = i + 1;
    }
    return sum;
}
```

The AST is:

```
Function: Main
├── Parameters:
└── Body:
│   ├── AssignStatement: Identifier("i")
│   │   ├── Value:
│   │   │   ├── Number(0)
│   ├── AssignStatement: Identifier("sum")
│   │   ├── Value:
│   │   │   ├── Number(0)
│   ├── WhileStatement
│   │   ├── Condition:
│   │   │   ├── Operator: Less
│   │   │   ├── Left:
│   │   │   │   ├── Identifier("i")
│   │   │   ├── Right:
│   │   │   │   ├── Number(10)
│   │   ├── Loop Body:
│   │   │   ├── AssignStatement: Identifier("sum")
│   │   │   │   ├── Value:
│   │   │   │   │   ├── Operator: Add
│   │   │   │   │   ├── Left:
│   │   │   │   │   │   ├── Identifier("sum")
│   │   │   │   │   ├── Right:
│   │   │   │   │   │   ├── Identifier("i")
│   │   │   ├── AssignStatement: Identifier("i")
│   │   │   │   ├── Value:
│   │   │   │   │   ├── Operator: Add
│   │   │   │   │   ├── Left:
│   │   │   │   │   │   ├── Identifier("i")
│   │   │   │   │   ├── Right:
│   │   │   │   │   │   ├── Number(1)
│   ├── ReturnStatement
│   │   ├── Value:
│   │   │   ├── Identifier("sum")
```

Also, Mikail if you see this, I KNOW ITS NOT FINISHED I WILL FINISH IT STOP BULLYING
