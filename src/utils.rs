pub(crate) fn extract_next_digits(s: &str) -> (&str, &str) {
    extract_until(|c| c.is_ascii_digit(), s)
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    extract_until(|c| c == ' ', s)
}

pub(crate) fn extract_operator(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" | "%" => {}
        _ => panic!("bad operator"),
    }

    (&s[0..1].trim(), &s[1..].trim())
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
}
