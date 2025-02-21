#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Delimiters
    Comma,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    SemiColon,
    DoubleQuote,
    LeftSquare,
    RightSquare,
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
    Dot,
    // Literals
    Integer(i32),
    Float(f64),
    Boolean(bool),
    Identifier(String),
    String(String),
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
    Append,
    Length,
}

impl Token {
    /// Attempts to parse a single token from the beginning of the input.
    /// Returns the token and the remaining input.

    pub fn new(input: &str) -> Result<(Self, &str), String> {
        let input = input.trim_start();
        if input.is_empty() {
            return Err("No input".into());
        }
        let first = input.chars().next().unwrap();

        //  Delimiters and string literals
        match first {
            '(' => return Ok((Token::LeftParen, &input[1..])),
            ')' => return Ok((Token::RightParen, &input[1..])),
            '{' => return Ok((Token::LeftBrace, &input[1..])),
            '}' => return Ok((Token::RightBrace, &input[1..])),
            '[' => return Ok((Token::LeftSquare, &input[1..])),
            ']' => return Ok((Token::RightSquare, &input[1..])),
            ';' => return Ok((Token::SemiColon, &input[1..])),
            ',' => return Ok((Token::Comma, &input[1..])),
            '"' => {
                if let Some(end) = input[1..].find('"') {
                    let literal = &input[1..1 + end];
                    let rest = &input[1 + end + 1..];
                    return Ok((Token::String(literal.to_string()), rest));
                } else {
                    return Err("Unterminated string literal".into());
                }
            }
            _ => {}
        }

        // Operators (check multi-character ones first)
        let op_candidates = ["!=", "==", ">=", "<=", "||", "&&"];
        for op in op_candidates.iter() {
            if input.starts_with(op) {
                let token = match *op {
                    "!=" => Token::NotEqual,
                    "==" => Token::EqualEqual,
                    ">=" => Token::GreaterEqual,
                    "<=" => Token::LessEqual,
                    "||" => Token::Or,
                    "&&" => Token::And,
                    _ => unreachable!(),
                };
                return Ok((token, &input[op.len()..]));
            }
        }

        // Number literals (both integer and float)
        if first.is_digit(10) {
            let mut idx = 0;
            let mut has_decimal = false;

            // Process all digits before potential decimal point
            while idx < input.len() && input.chars().nth(idx).unwrap().is_digit(10) {
                idx += 1;
            }

            // Check for decimal point followed by at least one digit
            if idx < input.len() && input.chars().nth(idx).unwrap() == '.' {
                if idx + 1 < input.len() && input.chars().nth(idx + 1).unwrap().is_digit(10) {
                    // This is a float with digits after decimal
                    has_decimal = true;
                    idx += 1; // Move past the decimal point

                    // Process all digits after decimal point
                    while idx < input.len() && input.chars().nth(idx).unwrap().is_digit(10) {
                        idx += 1;
                    }
                }
            }

            let number_str = &input[..idx];

            if has_decimal {
                // Parse as float
                let number = number_str
                    .parse::<f64>()
                    .map_err(|e| format!("Float parse error: {}", e))?;
                return Ok((Token::Float(number), &input[idx..]));
            } else {
                // Parse as integer
                let number = number_str
                    .parse::<i32>()
                    .map_err(|e| format!("Integer parse error: {}", e))?;
                return Ok((Token::Integer(number), &input[idx..]));
            }
        }

        // Single-character operators
        match first {
            '+' => return Ok((Token::Add, &input[1..])),
            '-' => return Ok((Token::Sub, &input[1..])),
            '*' => return Ok((Token::Mul, &input[1..])),
            '/' => return Ok((Token::Div, &input[1..])),
            '%' => return Ok((Token::Mod, &input[1..])),
            '>' => return Ok((Token::Greater, &input[1..])),
            '<' => return Ok((Token::Less, &input[1..])),
            '=' => return Ok((Token::Equal, &input[1..])),
            '!' => return Ok((Token::Not, &input[1..])),
            '.' => return Ok((Token::Dot, &input[1..])),
            _ => {}
        }

        // Identifiers and Keywords
        if first.is_alphabetic() {
            let mut idx = 0;
            for c in input.chars() {
                if c.is_alphanumeric() || c == '_' {
                    idx += 1;
                } else {
                    break;
                }
            }
            let ident = &input[..idx];
            // Match reserved keywords; otherwise, it's an identifier
            let token = match ident {
                "func" => Token::Func,
                "main" => Token::Main,
                "if" => Token::If,
                "else" => Token::Else,
                "for" => Token::For,
                "while" => Token::While,
                "let" => Token::Let,
                "return" => Token::Return,
                "print" => Token::Print,
                "true" => Token::Boolean(true),
                "false" => Token::Boolean(false),
                "append" => Token::Append,
                "length" => Token::Length,
                _ => Token::Identifier(ident.to_string()),
            };
            return Ok((token, &input[idx..]));
        }

        Err(format!("Unknown token starting with '{}'", first))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Delimiter & String Literal Tests ---
    #[test]
    fn test_left_paren() {
        assert_eq!(Token::new("("), Ok((Token::LeftParen, "")));
    }

    #[test]
    fn test_right_paren() {
        assert_eq!(Token::new(")"), Ok((Token::RightParen, "")));
    }

    #[test]
    fn test_left_brace() {
        assert_eq!(Token::new("{"), Ok((Token::LeftBrace, "")));
    }

    #[test]
    fn test_right_brace() {
        assert_eq!(Token::new("}"), Ok((Token::RightBrace, "")));
    }

    #[test]
    fn test_semicolon() {
        assert_eq!(Token::new(";"), Ok((Token::SemiColon, "")));
    }

    #[test]
    fn test_string_literal() {
        assert_eq!(
            Token::new("\"hello\""),
            Ok((Token::String("hello".to_string()), ""))
        );
    }

    #[test]
    fn test_unterminated_string() {
        assert!(Token::new("\"hello").is_err());
    }

    // --- Operator Tests ---
    #[test]
    fn test_add_operator() {
        assert_eq!(Token::new("  +  "), Ok((Token::Add, "  ")));
    }

    #[test]
    fn test_sub_operator() {
        assert_eq!(Token::new(" -"), Ok((Token::Sub, "")));
    }

    #[test]
    fn test_mul_operator() {
        assert_eq!(Token::new(" *"), Ok((Token::Mul, "")));
    }

    #[test]
    fn test_div_operator() {
        assert_eq!(Token::new(" /"), Ok((Token::Div, "")));
    }

    #[test]
    fn test_mod_operator() {
        assert_eq!(Token::new(" %"), Ok((Token::Mod, "")));
    }

    #[test]
    fn test_bang_equals_operator() {
        assert_eq!(Token::new("!="), Ok((Token::NotEqual, "")));
    }

    #[test]
    fn test_equal_equal_operator() {
        assert_eq!(Token::new("=="), Ok((Token::EqualEqual, "")));
    }

    #[test]
    fn test_greater_operator() {
        assert_eq!(Token::new(" >"), Ok((Token::Greater, "")));
    }

    #[test]
    fn test_greater_equal_operator() {
        assert_eq!(Token::new(">="), Ok((Token::GreaterEqual, "")));
    }

    #[test]
    fn test_less_operator() {
        assert_eq!(Token::new(" <"), Ok((Token::Less, "")));
    }

    #[test]
    fn test_less_equal_operator() {
        assert_eq!(Token::new("<="), Ok((Token::LessEqual, "")));
    }

    #[test]
    fn test_equal_operator() {
        assert_eq!(Token::new(" ="), Ok((Token::Equal, "")));
    }

    #[test]
    fn test_dot_operator() {
        assert_eq!(Token::new(" ."), Ok((Token::Dot, "")));
    }

    #[test]
    fn test_not_operator() {
        assert_eq!(Token::new(" !"), Ok((Token::Not, "")));
    }

    #[test]
    fn test_or_operator() {
        assert_eq!(Token::new("||"), Ok((Token::Or, "")));
    }

    #[test]
    fn test_and_operator() {
        assert_eq!(Token::new("&&"), Ok((Token::And, "")));
    }

    // --- Number Literal Tests ---
    #[test]
    fn test_number_literal() {
        assert_eq!(Token::new("3432"), Ok((Token::Integer(3432), "")));
    }

    // --- Keyword & Identifier Tests ---
    #[test]
    fn test_keyword_func() {
        assert_eq!(Token::new("func"), Ok((Token::Func, "")));
    }

    #[test]
    fn test_keyword_main() {
        assert_eq!(Token::new("main"), Ok((Token::Main, "")));
    }

    #[test]
    fn test_keyword_if() {
        assert_eq!(Token::new("if"), Ok((Token::If, "")));
    }

    #[test]
    fn test_keyword_else() {
        assert_eq!(Token::new("else"), Ok((Token::Else, "")));
    }

    #[test]
    fn test_keyword_for() {
        assert_eq!(Token::new("for"), Ok((Token::For, "")));
    }

    #[test]
    fn test_keyword_while() {
        assert_eq!(Token::new("while"), Ok((Token::While, "")));
    }

    #[test]
    fn test_keyword_let() {
        assert_eq!(Token::new("let"), Ok((Token::Let, "")));
    }

    #[test]
    fn test_keyword_return() {
        assert_eq!(Token::new("return"), Ok((Token::Return, "")));
    }

    #[test]
    fn test_keyword_print() {
        assert_eq!(Token::new("print"), Ok((Token::Print, "")));
    }

    #[test]
    fn test_boolean_true() {
        assert_eq!(Token::new("true"), Ok((Token::Boolean(true), "")));
    }

    #[test]
    fn test_boolean_false() {
        assert_eq!(Token::new("false"), Ok((Token::Boolean(false), "")));
    }

    #[test]
    fn test_identifier() {
        // If the identifier is not a keyword, it should be parsed as an identifier.
        assert_eq!(
            Token::new("variable"),
            Ok((Token::Identifier("variable".to_string()), ""))
        );
    }

    // --- Unknown Token Test ---
    #[test]
    fn test_unknown_token() {
        let err = Token::new("@");
        assert!(err.is_err());
    }
    #[test]
    fn test_float_literal() {
        assert_eq!(Token::new("3.14"), Ok((Token::Float(3.14), "")));
    }

    #[test]
    fn test_float_with_leading_decimal() {
        assert_eq!(Token::new(".25"), Ok((Token::Dot, "25")));
    }

    #[test]
    fn test_float_with_trailing_decimal() {
        assert_eq!(Token::new("42."), Ok((Token::Integer(42), ".")));
    }

    #[test]
    fn test_float_with_whitespace() {
        assert_eq!(Token::new("  3.14  "), Ok((Token::Float(3.14), "  ")));
    }
}
