#![warn(clippy::pedantic)]

fn encode_using_group_size(plain: &str, group_size: Option<usize>) -> String {
    let group_size = group_size.unwrap_or(std::usize::MAX);

    let mut s = plain
        .bytes()
        .filter_map(|c| 
            if      c.is_ascii_alphabetic() { Some(char::from(b'a' + b'z' - c.to_ascii_lowercase())) } 
            else if c.is_ascii_digit()      { Some(char::from(c)) } 
            else                            { None }
        )
        .enumerate()
        .fold(String::with_capacity(plain.len()), |mut encoded, (i, c)| {
            encoded.push(c);
            if i % group_size == (group_size - 1) { encoded += " "; }
            encoded
        });

    if let Some(last) = s.pop() { if last != ' ' { s.push(last) } }
    s
}

pub fn encode(plain: &str) -> String    { encode_using_group_size(plain, Some(5)) }
pub fn decode(cipher: &str) -> String   { encode_using_group_size(cipher, None) }
