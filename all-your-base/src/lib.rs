#![warn(clippy::all)]
#![warn(clippy::pedantic)]

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2    { return Err(Error::InvalidInputBase); }
    else if to_base < 2 { return Err(Error::InvalidOutputBase); }

    // First, figure out what number the collection of digits represents.
    let mut number = number.iter().try_fold(0, |n, &digit|
        if digit < from_base { Ok(n * from_base + digit) }
        else                 { Err(Error::InvalidDigit(digit)) }
    )?;

    if number == 0 { Ok(vec![]) } 
    else {
        // Calculate the number of digits in the final base.
        let num_final_digits = 1 + (f64::from(number)).log(f64::from(to_base)) as usize;

        // Allocate our final digits collection.
        let mut converted = vec![0; num_final_digits];

        // Set each digit.
        converted.iter_mut().rev().for_each(|place| { 
            *place = number % to_base; 
            number /= to_base; 
        });

        Ok(converted)
    }
}
