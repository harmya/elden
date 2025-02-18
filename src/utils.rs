pub(crate) fn extract_next_digit(s: &str) -> (&str, &str) {
    extract_until(|c| c.is_ascii_digit(), s, false)
}

pub(crate) fn extract_next_ident(s: &str) -> (&str, &str) {
    let s = s.trim();
    if s.chars().next().map_or(false, |c| c.is_ascii_alphabetic()) {
        extract_until(|c| c.is_ascii_alphanumeric() || c == '_', s, false)
    } else {
        ("", s.trim())
    }
}

pub(crate) fn extract_next_literal(s: &str) -> (&str, &str) {
    let s = s.trim();
    if s.chars()
        .next()
        .map_or(false, |c| (c.is_ascii_alphabetic() || c == ' '))
    {
        extract_until(|c| c.is_ascii_alphanumeric() || c == ' ', s, true)
    } else {
        ("", s)
    }
}

pub(crate) fn tag<'a, 'b>(starting_text: &'a str, s: &'b str) -> Result<&'b str, String> {
    if s.starts_with(starting_text) {
        Ok(&s[starting_text.len()..])
    } else {
        return Err(format!("expected {}", starting_text));
    }
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    extract_until(|c| c == ' ', s, false)
}

pub(crate) fn extract_operator_and_delimiter(s: &str) -> Result<(&str, &str), String> {
    if s.is_empty() {
        return Ok(("", ""));
    } else if s.len() == 1 {
        let (op, remainder) = (&s[0..1], &s[1..]);
        return Ok((op.trim(), remainder.trim()));
    } else {
        let (op, remainder) = match &s[0..2] {
            "==" | "!=" | "<=" | ">=" | "&&" | "||" => (&s[0..2], &s[2..]),
            _ => (&s[0..1], &s[1..]),
        };
        return Ok((op.trim(), remainder.trim()));
    }
}

fn extract_until(accept_char: impl Fn(char) -> bool, s: &str, is_literal: bool) -> (&str, &str) {
    let extracted_end = s
        .char_indices()
        .find_map(|(idx, c)| if accept_char(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    let extracted = &s[..extracted_end];
    let rest = &s[extracted_end..];

    if is_literal {
        (extracted, rest.trim())
    } else {
        (extracted.trim(), rest.trim())
    }
}

#[cfg(test)]
mod tests {

    use crate::token::Token;

    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_next_digit("1+2"), ("1", "+2"));
    }

    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_next_digit("10-20"), ("10", "-20"));
    }

    /* Some edge case testing */
    #[test]
    fn extract_nothing_from_empty_input() {
        assert_eq!(extract_next_digit(""), ("", ""));
    }

    #[test]
    fn extract_single_digitr() {
        assert_eq!(extract_next_digit("100"), ("100", ""));
    }

    #[test]
    fn extract_plus() {
        assert_eq!(extract_operator_and_delimiter("+2"), Ok(("+", "2")));
    }

    #[test]
    fn extract_opreator_nothing() {
        assert_eq!(extract_operator_and_delimiter(""), Ok(("", "")));
    }

    #[test]
    fn extract_minus() {
        assert_eq!(extract_operator_and_delimiter("-10"), Ok(("-", "10")));
    }

    #[test]
    fn extract_star() {
        assert_eq!(extract_operator_and_delimiter("*3"), Ok(("*", "3")));
    }

    #[test]
    fn extract_slash() {
        assert_eq!(extract_operator_and_delimiter("/4"), Ok(("/", "4")));
    }
    #[test]
    fn extract_equals_equals() {
        assert_eq!(extract_operator_and_delimiter("==5"), Ok(("==", "5")));
    }

    #[test]
    fn extract_not_equals() {
        assert_eq!(extract_operator_and_delimiter("!=6"), Ok(("!=", "6")));
    }

    #[test]
    fn extract_less_than_equals() {
        assert_eq!(extract_operator_and_delimiter("<=7"), Ok(("<=", "7")));
    }

    #[test]
    fn extract_greater_than_equals() {
        assert_eq!(extract_operator_and_delimiter(">=8"), Ok((">=", "8")));
    }

    #[test]
    fn extract_exclamation() {
        assert_eq!(extract_operator_and_delimiter("!9"), Ok(("!", "9")));
    }

    #[test]
    fn extract_equals() {
        assert_eq!(extract_operator_and_delimiter("=10"), Ok(("=", "10")));
    }

    #[test]
    fn extract_less_than() {
        assert_eq!(extract_operator_and_delimiter("<11"), Ok(("<", "11")));
    }

    #[test]
    fn extract_greater_than() {
        assert_eq!(extract_operator_and_delimiter(">12"), Ok((">", "12")));
    }

    #[test]
    fn extract_percent() {
        assert_eq!(extract_operator_and_delimiter("%13"), Ok(("%", "13")));
    }

    #[test]
    fn extract_left_paren() {
        assert_eq!(Token::new("  (4 + 4)"), Ok((Token::LeftParen, "4 + 4)")));
    }

    #[test]
    fn extract_right_paren() {
        assert_eq!(Token::new("  )"), Ok((Token::RightParen, "")));
    }

    #[test]
    fn extract_left_brace() {
        assert_eq!(Token::new(" {"), Ok((Token::LeftBrace, "")));
    }

    #[test]
    fn extract_right_brace() {
        assert_eq!(Token::new(" }"), Ok((Token::RightBrace, "")));
    }

    #[test]
    fn extract_alphanumeric_ident() {
        assert_eq!(extract_next_ident("diya = 20"), ("diya", "= 20"));
    }

    #[test]
    fn extract_alphanumeric_ident_with_whitespace() {
        assert_eq!(extract_next_ident("  mikail   = 20"), ("mikail", "= 20"));
    }

    #[test]
    fn extract_alphanumeric_ident_with_number() {
        assert_eq!(extract_next_ident("saad10 = 20"), ("saad10", "= 20"));
    }

    #[test]
    fn extract_alphanumeric_ident_with_number_and_whitespace() {
        assert_eq!(extract_next_ident("  saad10   = 20"), ("saad10", "= 20"));
    }
    #[test]
    fn tag_word() {
        assert_eq!(tag("let", "let a"), Ok(" a"));
    }
}
