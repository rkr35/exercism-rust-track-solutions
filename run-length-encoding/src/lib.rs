#![warn(clippy::all)]
#![warn(clippy::pedantic)]

#[derive(Debug, Default)]
struct Run {
    character: char,
    count: usize
}

mod encode;

pub fn encode(source: &str) -> String {
    const MAX_HEAP_ALLOC_BYTES: usize = 16;
    match encode::up_to_specified_heap_bytes(source, MAX_HEAP_ALLOC_BYTES) {
        Ok(encoded) => encoded,
        Err(e) => panic!("{}", e),
    }
}

pub fn decode(source: &str) -> String {
    use std::iter::repeat;

    let mut runs: [Run; 16] = Default::default();
    let mut runs_cursor = 0;
    let mut decoded_string_capacity = 0;

    for character in source.chars() {
        let mut run = runs.get_mut(runs_cursor).expect("Runs cursor out-of-bounds when decoding.");
        
        if character.is_ascii_digit() {
            let digit = character as u8 - b'0';
            run.count = 10 * run.count + usize::from(digit);
        } else {
            run.character = character;
            run.count = 1.max(run.count);
            decoded_string_capacity += run.count;
            runs_cursor += 1;
        }
    }

    let mut decoded = String::with_capacity(decoded_string_capacity);

    runs
        .iter()
        .flat_map(|run| repeat(run.character).take(run.count))
        .for_each(|character| decoded.push(character));

    if decoded.capacity() != decoded_string_capacity {
        panic!("The decoded string reallocated somewhere. Expected vs Actual capacity: {} vs {}", 
            decoded_string_capacity, decoded.capacity());
    }

    decoded
}
