#![warn(clippy::pedantic)]

pub fn encode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() {
        None
    } else {
        const A: u8 = b'a';
        let l = |c: u8| c.is_ascii_lowercase();
        let cipher = |c, k| char::from(A + (c + k - 2 * A) % 26);
        
        s
            .bytes()
            .zip(key.bytes().cycle())
            .map(|(c, k)| if l(c) && l(k) { Some(cipher(c, k)) } 
                          else            { None } )
            .collect()
    }
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
