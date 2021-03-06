#![warn(clippy::pedantic)]

const A: u8 = b'a';

pub fn code<F>(key: &str, s: &str, mut coder: F) -> Option<String>
where F: FnMut(u8, u8) -> u8 {
    if key.is_empty() {
        return None;
    }

    s.bytes()
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
    use rand::{distributions::Uniform, Rng};

    let key: String = rand::thread_rng()
        .sample_iter(Uniform::new_inclusive(b'a', b'z'))
        .take(100)
        .map(char::from)
        .collect();

    let encoded = encode(&key, s)
        .expect("encode_random() generated a key that contains a character outside of a-z.");

    (key, encoded)
}
