fn get_less_than_twenty_str(n: u64) -> &'static str {
    ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", 
     "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"]
        .get(n as usize)
        .expect("Received a number >= 20 in get_less_than_twenty_str()")
}

fn get_tens_str(tens_digit: u64) -> &'static str {
    ["twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"]
        .get(tens_digit.checked_sub(2).expect("Received a number < 2 in gen_tens_str()") as usize)
        .expect("Received a number > 9 OR number < 2 in get_tens_str()")
}

fn get_less_than_one_hundred_string(n: u64) -> String {
    let tens_digit = n / 10 % 10; 
    let mut s = get_tens_str(tens_digit).to_string();

    let ones_digit = n % 10;
    if ones_digit != 0 {
        s += "-";
        s += &get_simplest(ones_digit);
    }

    s
}

fn get_less_than_one_thousand_string(n: u64) -> String {
    let hundreds = n / 100;
    let mut s = get_simplest(hundreds) + " hundred";

    let tens_and_ones = n % 100;
    if tens_and_ones > 0 {
        s += " ";
        s += &get_simplest(tens_and_ones);
    }

    s
}

fn get_simplest(n: u64) -> String {
    match n {
        n if n < 20 => get_less_than_twenty_str(n).to_string(),
        n if n < 100 => get_less_than_one_hundred_string(n),
        n if n < 1000 => get_less_than_one_thousand_string(n),

        _ => "unimplemented".to_string()
    }
}

pub fn encode(n: u64) -> String {
    get_simplest(n).to_string()
}
