#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod encode;

pub fn encode(source: &str) -> String {
    const MAX_HEAP_ALLOC_BYTES: usize = 13;
    match encode::up_to_specificed_heap_bytes(source, MAX_HEAP_ALLOC_BYTES) {
        Ok(encoded) => encoded,
        Err(e) => panic!("{}", e)
    }
}

pub fn decode(source: &str) -> String {
    unimplemented!("Return the run-length decoding of {}.", source);
}
