#![warn(clippy::pedantic)]

pub fn encrypt(input: &str) -> String {
    fn normalize(c: char) -> Option<char> {
        if c.is_ascii_alphabetic() {
            Some(c.to_ascii_lowercase())
        } else {
            None
        }
    }

    let normalized_letters: String = input
        .chars()
        .filter_map(normalize)
        .collect();

    let [r, c] = {
        let number_letters = normalized_letters.len() as f64;
        let r = number_letters.sqrt().floor();
        let c = (number_letters / r).ceil();
        [r as usize, c as usize]
    };

    dbg!(r, c);
    normalized_letters
}
