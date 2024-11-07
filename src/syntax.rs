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
        Some('\\') => {
            match chars.next() {
                Some('u') => is_hex_num(chars, 4),
                Some('U') => is_hex_num(chars, 8),
                Some(c) => is_single_char_escape(c),
                _ => false
            }
        }
        Some('%') => is_hex_num(chars, 2),
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
    c.is_ascii_alphabetic() || c == '_' || c == '-' || c == '.'
}