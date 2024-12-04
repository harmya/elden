use crate::utils::extract_next_ident;

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Main,
    If,
    Else,
    True,
    False,
    For,
    While,
    Let,
    Return,
    Print,
    Func,
}

impl Keyword {
    pub fn new(s: &str) -> Result<(Self, &str), String> {
        let (keyword, rest) = extract_next_ident(s.trim());
        let keyword = match keyword {
            "func" => Self::Func,
            "main" => Self::Main,
            "if" => Self::If,
            "else" => Self::Else,
            "true" => Self::True,
            "false" => Self::False,
            "for" => Self::For,
            "while" => Self::While,
            "let" => Self::Let,
            "return" => Self::Return,
            "print" => Self::Print,
            _ => return Err(format!("Unrecognized Symbol: {}", keyword)),
        };
        Ok((keyword, rest))
    }

    pub fn tag(s: &str) -> bool {
        matches!(
            s,
            "main"
                | "if"
                | "else"
                | "true"
                | "false"
                | "for"
                | "while"
                | "let"
                | "return"
                | "print"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_main_keyword() {
        assert_eq!(Keyword::new("main()"), Ok((Keyword::Main, "()")));
    }

    #[test]
    fn parse_if_keyword() {
        assert_eq!(Keyword::new("if"), Ok((Keyword::If, "")));
    }

    #[test]
    fn parse_else_keyword() {
        assert_eq!(Keyword::new("else"), Ok((Keyword::Else, "")));
    }

    #[test]
    fn parse_true_keyword() {
        assert_eq!(Keyword::new("true"), Ok((Keyword::True, "")));
    }

    #[test]
    fn parse_false_keyword() {
        assert_eq!(Keyword::new("false"), Ok((Keyword::False, "")));
    }

    #[test]
    fn parse_for_keyword() {
        assert_eq!(Keyword::new("for"), Ok((Keyword::For, "")));
    }

    #[test]
    fn parse_while_keyword() {
        assert_eq!(Keyword::new("while"), Ok((Keyword::While, "")));
    }

    #[test]
    fn parse_let_keyword() {
        assert_eq!(Keyword::new("let"), Ok((Keyword::Let, "")));
    }

    #[test]
    fn parse_return_keyword() {
        assert_eq!(Keyword::new("return"), Ok((Keyword::Return, "")));
    }

    #[test]
    fn parse_print_keyword() {
        assert_eq!(Keyword::new("print"), Ok((Keyword::Print, "")));
    }

    #[test]
    #[should_panic(expected = "Unrecognized Symbol: invalid")]
    fn parse_unrecognized_keyword() {
        let err = Keyword::new("invalid").unwrap_err();
        panic!("{}", err);
    }
}
