#![warn(clippy::pedantic)]

pub fn encrypt(input: &str) -> String {
    let normalized_letters: Vec<_> = input
        .chars()
        .filter_map(|c| 
            if c.is_ascii_alphanumeric() { Some(c.to_ascii_lowercase()) }
            else                         { None }
        )
        .collect();

    let columns = (normalized_letters.len() as f64).sqrt().ceil() as usize;

    (0..columns)
        .map(|c|
            normalized_letters
                .chunks(columns)
                .map(|row| row.get(c).copied().unwrap_or(' '))
                .collect::<String>()
        )
        .collect::<Vec<_>>()
        .join(" ")
}
