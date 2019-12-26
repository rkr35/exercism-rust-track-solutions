/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const ALPHABET_SIZE: i32 = 26;

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

fn is_coprime(a: i32, b: i32) -> bool {
    gcd(a, b) == 1
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    use core::convert::TryInto;

    if !is_coprime(a, ALPHABET_SIZE) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let mut encoded = String::new();
    let mut num_added = 0;

    for c in plaintext.bytes() {
        if c.is_ascii_digit() {
            if num_added != 0 && num_added % 5 == 0 {
                encoded.push(' ');
            }

            encoded.push(c.into());
            num_added += 1;
        } else if c.is_ascii_alphabetic() {
            // E(x) = (ax + b) mod m
            let x = i32::from(c.to_ascii_lowercase() - b'a');

            let e: u8 = ((a*x + b) % ALPHABET_SIZE).try_into().expect("Encountered a non-ASCII alphabetic character");
            
            if num_added != 0 && num_added % 5 == 0 {
                encoded.push(' ');
            }

            encoded.push((b'a' + e).into());
            num_added += 1;
        }
    }

    Ok(encoded)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    unimplemented!("Decode {} with the key ({}, {})", ciphertext, a, b);
}
