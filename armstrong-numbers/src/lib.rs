fn get_num_digits(num: u32) -> u32 {
    1 + f64::from(num).log10().floor() as u32
}

fn get_digit(num: u32, ordinal: u32) -> u32 {
    num / 10_u32.pow(ordinal) % 10
}

pub fn is_armstrong_number(num: u32) -> bool {
    let num_digits = get_num_digits(num);

    (0..num_digits)
        .map(|i| get_digit(num, i).pow(num_digits))
        .sum::<u32>()
        == num
}
