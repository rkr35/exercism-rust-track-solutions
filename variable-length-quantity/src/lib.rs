use std::iter::once;

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

// The type of all input values.
type Value = u32;

// The type of all output encoded bytes.
type Byte = u8;

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[Value]) -> Vec<u8> {
    // The number of significants bits per encoded byte.
    const GROUP_SIZE: Value = 7;

    // A mask containing the position of the continuation bit.
    // Used to set the continuation bit for non-terminal encoded bytes.
    const CONTINUE_MASK: Value = (1 << GROUP_SIZE);

    // A mask containing the positions of the significant bits.
    // Used to extract a GROUP_SIZE chunk of information from the input
    // to place into an encoded byte.
    const GROUP_MASK: Value = CONTINUE_MASK - 1;

    // The number of bits per input value.
    // Used to determine how many groups of GROUP_SIZE the final encoding
    // should have for each input value.
    const BITS_PER_VALUE: Value = 8 * std::mem::size_of::<Value>() as Value;

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
            let get_group = move |i| (number >> (i * GROUP_SIZE) & GROUP_MASK) as Byte;

            // First encode, from right-to-left, all the groups that need a continuation
            // bit set.
            (1..number_of_groups)
                .rev()
                .map(move |i| get_group(i) | CONTINUE_MASK as Byte)

            // Then encode the last group, which we won't set the continuation bit for.
                .chain(once(get_group(0)))
        })
        .collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    unimplemented!("Convert the list of bytes {:?} to a list of numbers", bytes)
}
