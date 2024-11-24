pub(crate) fn extract_next_digits(s: &str) -> (&str, &str) {
    extract_until(|c| c.is_ascii_digit(), s)
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    extract_until(|c| c == ' ', s)
}

pub(crate) fn extract_operator(s: &str) -> (&str, &str) {
    if s.is_empty() {
        return ("", "");
    } else if s.len() == 1 {
        let (op, remainder) = match &s[0..1] {
            "+" | "-" | "*" | "/" | "%" | "!" => (&s[0..1], &s[1..]),
            "=" => (&s[0..1], &s[1..]),
            "<" | ">" => (&s[0..1], &s[1..]),
            _ => panic!("bad operator"),
        };
        return (op.trim(), remainder.trim());
    } else {
        let (op, remainder) = match &s[0..2] {
            "==" | "!=" | "<=" | ">=" => (&s[0..2], &s[2..]),
            _ => match &s[0..1] {
                "+" | "-" | "*" | "/" | "%" | "!" => (&s[0..1], &s[1..]),
                "=" => (&s[0..1], &s[1..]),
                "<" | ">" => (&s[0..1], &s[1..]),
                _ => panic!("bad operator"),
            },
        };
        return (op.trim(), remainder.trim());
    }
}

fn extract_until(accept_char: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let extracted_end = s
        .char_indices()
        .find_map(|(idx, c)| if accept_char(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    let extracted = &s[..extracted_end];
    let rest = &s[extracted_end..];

    (extracted.trim(), rest.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_one_digit() {
        assert_eq!(extract_next_digits("1+2"), ("1", "+2"));
    }

    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_next_digits("10-20"), ("10", "-20"));
    }

    /* Some edge case testing */
    #[test]
    fn extract_nothing_from_empty_input() {
        assert_eq!(extract_next_digits(""), ("", ""));
    }

    #[test]
    fn extract_single_digitr() {
        assert_eq!(extract_next_digits("100"), ("100", ""));
    }

    #[test]
    fn extract_plus() {
        assert_eq!(extract_operator("+2"), ("+", "2"));
    }

    #[test]
    fn extract_opreator_nothing() {
        assert_eq!(extract_operator(""), ("", ""));
    }

    #[test]
    fn extract_minus() {
        assert_eq!(extract_operator("-10"), ("-", "10"));
    }

    #[test]
    fn extract_star() {
        assert_eq!(extract_operator("*3"), ("*", "3"));
    }

    #[test]
    fn extract_slash() {
        assert_eq!(extract_operator("/4"), ("/", "4"));
    }
    #[test]
    fn extract_equals_equals() {
        assert_eq!(extract_operator("==5"), ("==", "5"));
    }

    #[test]
    fn extract_not_equals() {
        assert_eq!(extract_operator("!=6"), ("!=", "6"));
    }

    #[test]
    fn extract_less_than_equals() {
        assert_eq!(extract_operator("<=7"), ("<=", "7"));
    }

    #[test]
    fn extract_greater_than_equals() {
        assert_eq!(extract_operator(">=8"), (">=", "8"));
    }

    #[test]
    fn extract_exclamation() {
        assert_eq!(extract_operator("!9"), ("!", "9"));
    }

    #[test]
    fn extract_equals() {
        assert_eq!(extract_operator("=10"), ("=", "10"));
    }

    #[test]
    fn extract_less_than() {
        assert_eq!(extract_operator("<11"), ("<", "11"));
    }

    #[test]
    fn extract_greater_than() {
        assert_eq!(extract_operator(">12"), (">", "12"));
    }

    #[test]
    fn extract_percent() {
        assert_eq!(extract_operator("%13"), ("%", "13"));
    }
}
