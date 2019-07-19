#![warn(clippy::all)]
#![warn(clippy::pedantic)]

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits. However, your function must be able to
///     process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2    { return Err(Error::InvalidInputBase); }
    else if to_base < 2 { return Err(Error::InvalidOutputBase); }

    // First, figure out what number the collection of digits represents.
    let mut number = number.iter().try_fold(0, |n, &digit|
        if digit < from_base { Ok(n * from_base + digit) }
        else                 { Err(Error::InvalidDigit(digit)) }
    )?;

    if number == 0 {
        return Ok(vec![]);
    }

    // Calculate the number of digits in the final base.
    let num_final_digits = 1 + (f64::from(number)).log(f64::from(to_base)) as usize;

    // Allocate our collection of digits with the appropriate capacity.
    let mut converted = vec![0; num_final_digits];

    let mut cursor = converted.iter_mut().rev();

    // While there are parts of the number to convert...
    while number > 0 {
        // Get a mutable reference to the next digit place.
        let cursor = cursor.next().expect("Cursor is out-of-bounds");

        // Calculate the digit that should be in this place.
        let digit = number % to_base;
        
        // Set the digit.
        *cursor = digit;

        // Shift the number to the next digit place.
        number /= to_base;
    }

    Ok(converted)
}
