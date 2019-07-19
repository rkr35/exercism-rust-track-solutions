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

    // Returns the largest integer k such that to_base.pow(k) <= n.
    // Used to determine the farthest left digit of `number` in base `to_base`.
    let largest_power = |n| (f64::from(n)).log(f64::from(to_base)) as u32;

    // Calculate the number of digits in the final base.
    let mut power = largest_power(number);
    let num_final_digits = power as usize + (number > 0) as usize;

    // Allocate our collection of digits with the appropriate capacity.
    let mut converted = vec![0; num_final_digits];

    // While there are parts of the number to convert...
    while number > 0 {
        // Find the largest power that is <= the number.
        let largest_base_multiple = to_base.pow(power);

        // Find the digit in `to_base` that corresponds to the 
        // largest multiple of the largest power.
        let digit = number / largest_base_multiple;

        // Set that digit in our collection of digits.
        let digit_place = num_final_digits - power as usize - 1;
        converted[digit_place] = digit;

        // Remove the largest multiple from the number.
        number -= digit * largest_base_multiple;

        // Find the next largest power that fits in whatever is left after
        // the subtraction.
        power = largest_power(number);
    }

    Ok(converted)
}
