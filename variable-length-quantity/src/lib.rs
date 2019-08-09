#![warn(clippy::all)]
#![warn(clippy::pedantic)]

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

// The type of all input decoded values.
type Value = u32;

// The type of all output encoded bytes.
type Byte = u8;

// The number of significants bits per encoded byte.
const GROUP_SIZE: Value = 7;

// A mask containing the position of the continuation bit.
// Used to set the continuation bit for non-terminal encoded bytes.
const CONTINUE_MASK: Value = 1 << GROUP_SIZE;

// A mask containing the positions of the significant bits.
// Used to extract a GROUP_SIZE chunk of information from the input
// to place into an encoded byte, or to extract a chunk of information from an
// encoded byte to place into a decoded byte.
const GROUP_MASK: Value = CONTINUE_MASK - 1;

// The number of bits per input value.
// Used to determine how many groups of GROUP_SIZE the final encoding
// should have for each input value.
const BITS_PER_VALUE: Value = (0 as Value).count_zeros();

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[Value]) -> Vec<Byte> {
    values
        .iter()
        .flat_map(|&number| {
            // Find the number of GROUP_SIZE groups for this input value.
            let number_of_groups = {
                let number_of_significant_bits = BITS_PER_VALUE - number.leading_zeros();
                let mut n = number_of_significant_bits / GROUP_SIZE;
            
                // If the number of significant bits is not a multiple of GROUP_SIZE,
                // we'll need to add one more group to hold the last few bits.
                if number_of_significant_bits % GROUP_SIZE != 0 {
                    n += 1;
                }

                n
            };

            // A helper fuction that extracts the i'th group of GROUP_SIZE bits from `number`.
            let get_group = move |i| {
                let group = number >> (i * GROUP_SIZE);
                let significant_bits = group & GROUP_MASK;
                significant_bits as Byte
            };

            // First encode, from right-to-left, all the groups that need a continuation
            // bit set.
            (1..number_of_groups)
                .rev()
                .map(move |i| get_group(i) | CONTINUE_MASK as Byte)

            // Then encode the last group, which we won't set the continuation bit for.
                .chain(std::iter::once(get_group(0)))
        })
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    // Holds the numbers we decode from the byte stream.
    let mut decoded_numbers = vec![];

    // Used to hold results of decoding a single number.
    // Variant `Some` when the accumulator is currently being used to decode a number, i.e.,
    // the below algorithm has not reached the terminal byte.
    // Variant `None` when not decoding a number.
    let mut accumulator = None;

    for byte in bytes {
        let (is_terminal, decoded_number_so_far) = {
            // Ensure the accumulator is initialized.
            let accumulator = accumulator.get_or_insert(0 as Value);
            
            // Extract a piece of the encoded number.
            let payload = byte & GROUP_MASK as Byte;

            // Make room in the accumulator for that piece by shifting left GROUP_SIZE bits.
            // We shift left by multiplying by 2^GROUP_SIZE so that we can use checked_mul()
            // to check for an overflow.
            *accumulator = (*accumulator)
                .checked_mul(CONTINUE_MASK)
                .ok_or(Error::Overflow)?;

            // Add that piece to the accumulator.
            *accumulator |= u32::from(payload);

            // We know the sign-bit (the leftmost bit) will be `1` if
            // we are expecting more bytes and `0` if we are done decoding
            // this number, i.e., we reached the "terminal" byte.
            let sign_bit = byte >> GROUP_SIZE;
            let is_terminal = sign_bit == 0;

            (is_terminal, *accumulator)
        };

        // Check if we are expecting more pieces for this encoded number.
        if is_terminal {
            // If not, then we fully decoded the number. 
            // We add this decoded number to our collection.
            decoded_numbers.push(decoded_number_so_far);

            // We reset the accumulator to the `None` variant to signify that we are done
            // using it for decoding this number.
            accumulator = None;
        }
    }

    // Did we stop in the middle of decoding a number?
    if accumulator.is_some() {
        Err(Error::IncompleteNumber)
    } else {
        Ok(decoded_numbers)
    }
}
