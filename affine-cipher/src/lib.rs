use core::iter::{FromIterator, IntoIterator};

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const ALPHABET_SIZE: i32 = 26;

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    use core::convert::TryInto;

    if !is_coprime(a, ALPHABET_SIZE) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    Ok(plaintext
        .bytes()
        .filter_map(|c| match c {
            c if c.is_ascii_digit() => Some(c),
            c if c.is_ascii_alphabetic() => {
                let x = i32::from(c.to_ascii_lowercase() - b'a');
                let e = (a*x + b) % ALPHABET_SIZE;
                let e: u8 = e.try_into().expect("Encountered a non-ASCII alphabetic character");
                Some(b'a' + e)
            },
            _ => None,
        })
        .collect::<Encoded>()
        .into())
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    unimplemented!("Decode {} with the key ({}, {})", ciphertext, a, b);
}

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

#[derive(Default)]
struct Encoded {
    encoded: String,
    num_added: usize,
}

impl Encoded {
    const GROUP_SIZE: usize = 5;

    fn push(&mut self, c: u8) {
        self.add_space();
        self.encoded.push(c.into());
        self.num_added += 1;
    }

    fn add_space(&mut self) {
        if self.needs_space() {
            self.encoded.push(' ');
        }
    }

    fn needs_space(&self) -> bool {
        self.num_added > 0 && self.num_added % Self::GROUP_SIZE == 0
    }
}

impl FromIterator<u8> for Encoded {
    fn from_iter<I: IntoIterator<Item=u8>>(iter: I) -> Self {
        let mut encoded = Self::default();

        for c in iter {
            encoded.push(c);
        }

        encoded
    }
}

impl Into<String> for Encoded {
    fn into(self) -> String {
        self.encoded
    }
}
