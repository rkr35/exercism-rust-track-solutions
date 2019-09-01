#![warn(clippy::pedantic)]

pub fn encrypt(input: &str) -> String {
    use std::iter::once;

    let normalized_letters: Vec<_> = input
        .chars()
        .filter_map(|c| 
            if c.is_ascii_alphanumeric() { Some(c.to_ascii_lowercase()) }
            else                         { None }
        )
        .collect();

    let columns = (normalized_letters.len() as f64).sqrt().ceil() as usize;

    let mut encrypted: String = (0..columns)
        .flat_map(|column|
            normalized_letters
                .chunks(columns)
                .map(move |row| row.get(column).copied().unwrap_or(' '))
                .chain(once(' '))
        )
        .collect();

    encrypted.pop();
    encrypted
}
