#![warn(clippy::all)]
#![warn(clippy::pedantic)]

#[derive(Debug, Default)]
struct Run {
    character: u8,
    count: usize
}

fn write_digits(destination: &mut String, mut number: usize) {
    let (digits, num_free_slots_left) = {
        const MAX_NUMBER_OF_DIGITS: usize = 2;
        let mut buffer = [0; MAX_NUMBER_OF_DIGITS];
        let mut num_added = 0;

        while number > 0 {
            let slot = buffer.get_mut(num_added).expect("Exceeded digits upper-bound");
            *slot = number % 10;
            num_added += 1;
            number /= 10;
        }

        if destination.len() + num_added > destination.capacity() {
            panic!("Writing {} digits will cause the {}/{} byte buffer to reallocate.",
                num_added, destination.len(), destination.capacity());
        }

        (buffer, buffer.len() - num_added)
    };

    for &digit in digits.iter().rev().skip(num_free_slots_left) {
        destination.push_str(["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"][digit]);
    }
}

fn write(destination: &mut String, source: &Run) {
    if source.count > 1 {
        write_digits(destination, source.count);
    }
    
    if destination.len() + 1 > destination.capacity() {
        panic!("Writing one more letter will cause the {} byte buffer to reallocate.",
            destination.capacity());
    }

    let character = [source.character];
    let character = std::str::from_utf8(&character)
        .expect("Error converting a single u8 Run character into &str");

    destination.push_str(character);
}

pub fn encode(source: &str) -> String {
    const MAX_HEAP_ALLOC_BYTES: usize = 14;

    let mut bytes = source.bytes();
    let current = bytes.nth(0).map(|character| Run { character, count: 1 });

    if current.is_none() {
        return String::new();
    }

    let mut current = current.expect("Tried to unwrap empty current Run");
    let mut encoded = String::with_capacity(MAX_HEAP_ALLOC_BYTES);

    for character in bytes {
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
