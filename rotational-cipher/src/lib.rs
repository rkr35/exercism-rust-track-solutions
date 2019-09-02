#![warn(clippy::pedantic)]
pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| if c.is_alphabetic() {
            let first = if c.is_ascii_uppercase() { b'A' } else { b'a' } as i8;
            let negative_mod = |n, m| ((n % m) + m) % m;
            let old_position = c as i8 - first;
            let new_position = negative_mod(old_position + key, 26);
            (first + new_position) as u8 as char
        } else {
            c
        })
        .collect()  
}