pub enum ParseErr {
    NotComplete,
    NoMatch(String)
}

pub fn word(s: &[u8]) -> Result<(&[u8], &[u8]), ParseErr> {
    if s.len() == 0 {
        return Err(ParseErr::NotComplete);
    }

    if !s[0].is_ascii_alphabetic() {
        return Err(ParseErr::NoMatch("word Doesn't start with alpha".to_string()));
    }

    let mut i = 1;

    while i < s.len() {
        if s[i].is_ascii_alphanumeric() || s[i] == b'_' {
            i += 1;
        } else if s[i].is_ascii_whitespace() {
            return Ok((&s[i..], &s[0..i]));
        } else {
            return Err(ParseErr::NoMatch("word contains invalid chars".to_string()));
        }
    }

    Err(ParseErr::NotComplete)
}
