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
}

impl Keyword {
    pub fn new(s: &str) -> (Self, &str) {
        let (keyword, rest) = extract_next_ident(s.trim());
        let keyword = match keyword {
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
            _ => panic!("Unrecognized Symbol: {}", keyword),
        };
        (keyword, rest)
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
        assert_eq!(Keyword::new("main()"), (Keyword::Main, "()"));
    }

    #[test]
    fn parse_if_keyword() {
        assert_eq!(Keyword::new("if"), (Keyword::If, ""));
    }

    #[test]
    fn parse_else_keyword() {
        assert_eq!(Keyword::new("else"), (Keyword::Else, ""));
    }

    #[test]
    fn parse_true_keyword() {
        assert_eq!(Keyword::new("true"), (Keyword::True, ""));
    }

    #[test]
    fn parse_false_keyword() {
        assert_eq!(Keyword::new("false"), (Keyword::False, ""));
    }

    #[test]
    fn parse_for_keyword() {
        assert_eq!(Keyword::new("for"), (Keyword::For, ""));
    }

    #[test]
    fn parse_while_keyword() {
        assert_eq!(Keyword::new("while"), (Keyword::While, ""));
    }

    #[test]
    fn parse_let_keyword() {
        assert_eq!(Keyword::new("let"), (Keyword::Let, ""));
    }

    #[test]
    fn parse_return_keyword() {
        assert_eq!(Keyword::new("return"), (Keyword::Return, ""));
    }

    #[test]
    fn parse_print_keyword() {
        assert_eq!(Keyword::new("print"), (Keyword::Print, ""));
    }

    #[test]
    #[should_panic(expected = "Unrecognized Symbol: invalid")]
    fn parse_unrecognized_keyword() {
        Keyword::new("invalid");
    }
}
