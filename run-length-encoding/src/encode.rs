#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use crate::Run;

#[derive(Debug)]
pub struct ReallocateError {
    capacity: usize,
    current: usize,
    planned_to_add: usize
}

#[derive(Debug)]
pub enum Error {
    DigitsReallocates(ReallocateError),
    
    CharacterReallocates {
        character: char,
        reallocate_error: ReallocateError
    },

    DigitsExceededUpperBound {
        whats_left: usize,
        num_added: usize,
        upper_bound: usize,
    },

    FinalResultReallocated {
        intended_final_capacity: usize, 
        actual_final_capacity: usize
    },
}

use std::fmt::{Display, Formatter, Result as FmtResult};

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:#?}", self)
    }
}

const DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn digits(destination: &mut String, mut number: usize) -> Result<(), Error> {
    let (digits, num_free_slots_left) = {
        const MAX_NUMBER_OF_DIGITS: usize = 3;
        let mut buffer = [0; MAX_NUMBER_OF_DIGITS];
        let mut num_added = 0;

        while number > 0 {
            let slot = buffer
                .get_mut(num_added)
                .ok_or(Error::DigitsExceededUpperBound {
                    whats_left: number,
                    num_added,
                    upper_bound: MAX_NUMBER_OF_DIGITS
                })?;

            *slot = number % 10;
            num_added += 1;
            number /= 10;
        }

        if destination.len() + num_added > destination.capacity() {
            let error = Error::DigitsReallocates(ReallocateError { 
                capacity: destination.capacity(),
                current: destination.len(),
                planned_to_add: num_added
            });

            return Err(error);
        }

        (buffer, buffer.len() - num_added)
    };

    for &digit in digits.iter().rev().skip(num_free_slots_left) {
        destination.push_str(DIGITS[digit]);
    }

    Ok(())
}

fn to(destination: &mut String, source: &Run) -> Result<(), Error> {
    if source.count > 1 {
        digits(destination, source.count)?;
    }
    
    if destination.len() + 1 > destination.capacity() {
        let error = Error::CharacterReallocates {
            character: source.character,
            reallocate_error: ReallocateError { 
                capacity: destination.capacity(),
                current: destination.len(),
                planned_to_add: 1
            }
        };
        
        return Err(error);
    }

    destination.push(source.character);

    Ok(())
}

pub fn up_to_specified_heap_bytes(source: &str, max_heap_alloc_bytes: usize) -> Result<String, Error> {
    let mut chars = source.chars();
    let current = chars.nth(0).map(|character| Run { character, count: 1 });

    if current.is_none() {
        return Ok(String::new());
    }

    let mut current = current.expect("Tried to unwrap empty current Run");
    let mut encoded = String::with_capacity(max_heap_alloc_bytes);

    for character in chars {
        if current.character == character {
            current.count += 1;
        } else {
            to(&mut encoded, &current)?;
            current = Run {character, count: 1};
        }
    }

    to(&mut encoded, &current)?;

    if encoded.capacity() <= max_heap_alloc_bytes {
        Ok(encoded)
    } else {
        Err(Error::FinalResultReallocated {
            intended_final_capacity: max_heap_alloc_bytes,
            actual_final_capacity: encoded.capacity()
        })
    }
}