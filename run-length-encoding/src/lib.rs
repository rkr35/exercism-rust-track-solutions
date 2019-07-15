#![warn(clippy::all)]
#![warn(clippy::pedantic)]

const MAX_HEAP_ALLOC_BYTES: usize = 16;

#[derive(Debug, Default)]
struct Run {
    character: u8,
    count: usize
}

fn write(destination: &mut String, source: &Run) {
    macro_rules! panic_if_reallocates {
        ($num_bytes:expr) => {
            if destination.len() + $num_bytes > destination.capacity() {
                panic!("write(): writing {} bytes will cause a reallocation to the original buffer of {}/{} bytes",
                    $num_bytes, destination.len(), destination.capacity());
            }
        };
    }

    if source.count > 1 {
        let num_digits = 1 + (source.count as f64)
            .log10()
            .floor() as usize;

        panic_if_reallocates!(num_digits);

        (0..num_digits)
            .map(|i| source.count / 10_usize.pow(i as u32) % 10)
            .map(|digit| ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"][digit])
            .rev()
            .for_each(|digit| destination.push_str(digit));
    }

    panic_if_reallocates!(1);

    destination
        .push_str(std::str::from_utf8(&[source.character])
        .expect("Error converting a single u8 Run character into &str"));
}

pub fn encode(source: &str) -> String {
    let current = source.bytes().nth(0).map(|character| Run { character, count: 1 });

    if current.is_none() {
        return String::new();
    }

    let mut current = current.expect("Tried to unwrap empty current Run");
    let mut encoded = String::with_capacity(MAX_HEAP_ALLOC_BYTES);

    for character in source.bytes().skip(1) {
        if current.character == character {
            current.count += 1;
        } else {
            write(&mut encoded, &current);
            current = Run {character, count: 1};
        }
    }

    write(&mut encoded, &current);

    encoded
}

pub fn decode(source: &str) -> String {
    unimplemented!("Return the run-length decoding of {}.", source);
}
