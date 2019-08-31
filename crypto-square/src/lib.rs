#![warn(clippy::pedantic)]

pub fn encrypt(input: &str) -> String {
    fn normalize(c: char) -> Option<char> {
        if c.is_ascii_alphanumeric() {
            Some(c.to_ascii_lowercase())
        } else {
            None
        }
    }

    let normalized_letters: Vec<_> = input
        .chars()
        .filter_map(normalize)
        .collect();

    let [rows, columns] = {
        let number_letters = normalized_letters.len() as f64;
        let r = number_letters.sqrt().floor();
        let c = (number_letters / r).ceil();
        [r as usize, c as usize]
    };

    let mut encrypted = String::with_capacity(columns * (rows + 1));

    for c in 0..columns {
        for r in 0..rows {
            let position = r * columns + c;

            let letter = normalized_letters
                .get(position)
                .copied()
                .unwrap_or(' ');

            encrypted.push(letter);
        }

        if c != columns - 1 {
            encrypted.push(' ');
        }
    }
    
    encrypted
}
