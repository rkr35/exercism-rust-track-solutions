#![warn(clippy::pedantic)]

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() {
        return None;
    }
    
    let mut encoded = String::with_capacity(s.len());

    let s = s
        .bytes()
        .zip(key.bytes().cycle());

    for (s, k) in s {
        const A: u8 = b'a';

        if !s.is_ascii_lowercase() || !k.is_ascii_lowercase() {
            return None;
        }

        let s = s - A; 
        let k = k - A; 
        let s = (s + k) % 26;
        let s = s + A;

        encoded.push(s.into());
    }

    Some(encoded)
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    unimplemented!("Use {} to decode {} using shift cipher", key, s)
}

pub fn encode_random(s: &str) -> (String, String) {
    unimplemented!(
        "Generate random key with only a-z chars and encode {}. Return tuple (key, encoded s)",
        s
    )
}
