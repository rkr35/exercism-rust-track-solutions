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
                let e = (a * x + b) % ALPHABET_SIZE;
                let e: u8 = e
                    .try_into()
                    .expect("Encountered a non-ASCII alphabetic character");
                Some(b'a' + e)
            }
            _ => None,
        })
        .collect::<Encoded<AddSpace>>()
        .into())
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    use core::convert::TryInto;

    let ExtendedGcd { gcd, x } = extended_gcd(a, ALPHABET_SIZE);
    
    if gcd != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    Ok(ciphertext
        .bytes()
        .filter_map(|c| match c {
            c if c.is_ascii_digit() => Some(c),
            c if c.is_ascii_alphabetic() => {
                let y = i32::from(c.to_ascii_lowercase() - b'a');
                let d = (x * (y - b)).rem_euclid(ALPHABET_SIZE);
                let d: u8 = d
                    .try_into()
                    .expect("Encountered a non-ASCII alphabetic character");
                Some(b'a' + d)
            }
            _ => None,
        })
        .collect::<Encoded<NoSpace>>()
        .into())
}

struct ExtendedGcd {
    gcd: i32,
    x: i32,
}

fn extended_gcd(a: i32, b: i32) -> ExtendedGcd {
    let mut k = [[a, b], [1, 0]];

    while k[0][1] != 0 {
        let q = k[0][0] / k[0][1];

        for [old, current] in k.iter_mut() {
            *old = core::mem::replace(current, *old - *current * q);
        }
    }

    let [[gcd, _], [x, _]] = k;
    ExtendedGcd { gcd, x }
}

fn is_coprime(a: i32, b: i32) -> bool {
    extended_gcd(a, b).gcd == 1
}

#[derive(Default)]
struct Encoded<Spacing> {
    encoded: String,
    num_added: usize,
    _spacing: Spacing,
}

impl Encoded<AddSpace> {
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

impl Encoded<NoSpace> {
    fn push(&mut self, c: u8) {
        self.encoded.push(c.into());
    }
}

impl<Spacing> Into<String> for Encoded<Spacing> {
    fn into(self) -> String {
        self.encoded
    }
}

macro_rules! impl_zsts {
    ($($zst:ident),+) => {
        $(
            #[derive(Default)]
            struct $zst;

            impl FromIterator<u8> for Encoded<$zst> {
                fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
                    let mut encoded = Self::default();
            
                    for c in iter {
                        encoded.push(c);
                    }
            
                    encoded
                }
            }
        )+
    }
}

impl_zsts!(AddSpace, NoSpace);
