/// "Encipher" with the Atbash cipher.
fn encode_using_group_size(plain: &str, group_size: Option<usize>) -> String {
    let group_size = group_size.unwrap_or(std::usize::MAX);

    let mut s = plain
        .bytes()
        .filter_map(|c| if c.is_ascii_alphabetic() {
            // Map characters to their partner on the opposite end of the
            // alphabet.
            Some(char::from(b'a' + b'z' - c.to_ascii_lowercase()))
        } else if c.is_ascii_digit() {
            // Pass digits untouched.
            Some(char::from(c))
        } else {
            // Ignore all other character classes.
            None
        })
        .enumerate()
        .fold(String::with_capacity(plain.len()), |mut encoded, (i, c)| {
            encoded.push(c);

            // If we're at the end of a group...
            if i % group_size == (group_size - 1) {
                // ...then append a space.
                encoded += " ";
            }

            encoded
        });

    if let Some(last) = s.pop() {
        // If we have an extraneous space, then remove it.
        // Otherwise, restore the character we popped.
        if last != ' ' {
            s.push(last)
        }
    }

    s
}

pub fn encode(plain: &str) -> String {
    encode_using_group_size(plain, Some(5))
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    encode_using_group_size(cipher, None)
}
