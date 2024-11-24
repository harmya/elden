pub(crate) fn extract_next_digit(s: &str) -> (&str, &str) {
    extract_until(|c| c.is_ascii_digit(), s)
}

pub(crate) fn extract_next_ident(s: &str) -> (&str, &str) {
    let s = s.trim();
    if s.chars().next().map_or(false, |c| c.is_ascii_alphabetic()) {
        extract_until(|c| c.is_ascii_alphanumeric() || c == '_', s)
    } else {
        ("", s.trim())
    }
}

pub(crate) fn tag<'a, 'b>(starting_text: &'a str, s: &'b str) -> Result<&'b str, String> {
    if s.starts_with(starting_text) {
        Ok(&s[starting_text.len()..])
    } else {
        Err(format!("expected {}", starting_text))
    }
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    extract_until(|c| c == ' ', s)
}

pub(crate) fn extract_operator_and_delimiter(s: &str) -> (&str, &str) {
    if s.is_empty() {
        return ("", "");
    } else if s.len() == 1 {
        let (op, remainder) = match &s[0..1] {
            "+" | "-" | "*" | "/" | "%" | "!" | "(" | ")" | "{" | "}" | "," | "." => {
                (&s[0..1], &s[1..])
            }
            "=" => (&s[0..1], &s[1..]),
            "<" | ">" => (&s[0..1], &s[1..]),
            _ => panic!("bad operator"),
        };
        return (op.trim(), remainder.trim());
    } else {
        let (op, remainder) = match &s[0..2] {
            "==" | "!=" | "<=" | ">=" => (&s[0..2], &s[2..]),
            _ => match &s[0..1] {
                "+" | "-" | "*" | "/" | "%" | "!" | "(" | ")" | "{" | "}" | "," | "." => {
                    (&s[0..1], &s[1..])
                }
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
