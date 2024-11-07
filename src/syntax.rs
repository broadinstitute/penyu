use strey::iter::Chars;

pub fn is_valid_local(local: &Chars) -> bool {
    let mut chars = local.clone();
    match chars.next() {
        Some('\\') => { if !is_escape_sequence(&mut chars) { return false; } }
        Some('%') => { if !is_hex_num(&mut chars, 2) { return false; } }
        Some(c) if is_local_name_first_char(c) => {}
        _ => { return false; }
    }
    while let Some(c) = chars.next() {
        match c {
            '\\' => { if !is_escape_sequence(&mut chars) { return false; } }
            '%' => { if !is_hex_num(&mut chars, 2) { return false; } }
            _ if is_local_name_later_char(c) => {}
            _ => { return false; }
        }
    }
    true
}

fn is_escape_sequence(chars: &mut Chars) -> bool {
    match chars.next() {
        Some('u') => is_hex_num(chars, 4),
        Some('U') => is_hex_num(chars, 8),
        Some(c) => is_single_char_escape(c),
        _ => false
    }
}

fn is_hex_num(chars: &mut Chars, len: usize) -> bool {
    for _ in 0..len {
        match chars.next() {
            Some(c) if c.is_ascii_hexdigit() => {}
            _ => { return false; }
        }
    }
    true
}

fn is_single_char_escape(c: char) -> bool {
    c == 't' || c == 'b' || c == 'n' || c == 'r' || c == 'f' || c == '\'' || c == '"' || c == '\\'
}

fn is_local_name_first_char(c: char) -> bool {
    is_local_name_later_char(c) || c == ':'
}

fn is_local_name_later_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '-' || c == '.'
}
pub fn encode_local_name<I: Iterator<Item=char>>(chars: &mut I) -> String {
    let mut encoded = String::new();
    if let Some(first) = chars.next() {
        if is_local_name_first_char(first) {
            encoded.push(first);
        } else {
            encoded.push_str(&encode_char(first));
        }
        for c in chars {
            if is_local_name_later_char(c) {
                encoded.push(c);
            } else {
                encoded.push_str(&encode_char(c));
            }
        }
    }
    encoded
}

pub fn encode_char(c: char) -> String {
    match c {
        '\t' => "\\t".to_string(),
        '\n' => "\\n".to_string(),
        '\r' => "\\r".to_string(),
        '\x0C' => "\\f".to_string(),
        '\'' => "\\'".to_string(),
        '"' => "\\\"".to_string(),
        '\\' => "\\\\".to_string(),
        _ => {
            let c = c as u32;
            if c < 256 {
                format!("%{:02X}", c)
            } else if c < 256 * 256 {
                format!("\\u{:04X}", c)
            } else {
                format!("\\U{:08X}", c)
            }
        }
    }
}