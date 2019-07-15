pub fn encode(source: &str) -> String {
    let mut current_run_character = None;
    let mut count = 0_usize;
    let mut encoded = String::new();

    fn encode_current(encoded: &mut String, current: char, count: usize) {
        if count > 1 {
            encoded.push_str(&count.to_string());
        }
        
        encoded.push_str(&current.to_string());
    }

    for character in source.chars() {
        match current_run_character {
            Some(current) => {
                if character == current {
                    count += 1;
                } else {
                    encode_current(&mut encoded, current, count);
                    current_run_character = Some(character);
                    count = 1;
                }
            },

            None => {
                current_run_character = Some(character);
                count = 1;
            }
        }
    }

    if let Some(current) = current_run_character {
        encode_current(&mut encoded, current, count);
    }

    encoded
}

pub fn decode(source: &str) -> String {
    unimplemented!("Return the run-length decoding of {}.", source);
}
