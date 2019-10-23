#![warn(clippy::pedantic)]

const A: u8 = b'a';

pub fn code<F>(key: &str, s: &str, mut coder: F) -> Option<String> 
where F: FnMut(u8, u8) -> u8 {
    if key.is_empty() {
        return None;
    }
    
    s
        .bytes()
        .zip(key.bytes().cycle())
        .map(|(c, k)| if c.is_ascii_lowercase() && k.is_ascii_lowercase() {
            Some((A + coder(c, k) % 26) as char)
        } else {
            None
        })
        .collect()
}

pub fn encode(key: &str, s: &str) -> Option<String> {
    code(key, s, |c, k| c + k - 2 * A)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    code(key, s, |c, k| c + 26 - k)
}

pub fn encode_random(s: &str) -> (String, String) {
    unimplemented!(
        "Generate random key with only a-z chars and encode {}. Return tuple (key, encoded s)",
        s
    )
}
