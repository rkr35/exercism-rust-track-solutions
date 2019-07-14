#![warn(clippy::all)]
#![warn(clippy::pedantic)]

fn get_ones_str(n: u64) -> &'static str {
    ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"]
        .get(n.checked_sub(1).expect("Received 0 in get_ones_str()") as usize)
        .expect("Received a number greater >= 10 in get_ones_str()")
}

fn get_less_than_twenty_str(n: u64) -> &'static str {
    ["ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"]
        .get(n.checked_sub(10).expect("Received a number <= 9 in get_less_than_twenty_str()") as usize)
        .expect("Received a number >= 20 in get_less_than_twenty_str()")
}

fn get_tens_str(tens_digit: u64) -> &'static str {
    ["twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"]
        .get(tens_digit.checked_sub(2).expect("Received a number <= 1 in gen_tens_str()") as usize)
        .expect("Received a number >= 10 in get_tens_str()")
}

type MaybeStr = Option<&'static str>;

#[derive(Debug, Default)]
struct Written {
    hundreds: MaybeStr,
    tens: MaybeStr,
    ones: MaybeStr,
}

#[derive(Debug, Default)]
struct Segment {
    written: Written,
    scale: MaybeStr,
}

impl From<u64> for Written {
    fn from(n: u64) -> Self {
        if n >= 1000 {
            panic!("Written::from(n) received an n >= 1000");
        }

        let mut ret = Self::default();

        let hundreds = n / 100;

        if hundreds > 0 {
            ret.hundreds = Some(get_ones_str(hundreds));
        }

        let tens = n % 100;

        if tens >= 10 && tens < 20 {
            ret.tens = Some(get_less_than_twenty_str(tens));
            return ret;
        }

        let tens = n / 10 % 10;

        ret.tens = match tens {
            2...9 => Some(get_tens_str(tens)),
            _ => None
        };

        let ones = n % 10;

        ret.ones = match ones {
            1...9 => Some(get_ones_str(ones)),
            _ => None
        };

        ret
    }
}

pub fn encode(mut n: u64) -> String {
    const NUM_SCALES: usize = 6;
    const SCALES: [&str; NUM_SCALES] = ["thousand", "million", "billion", "trillion", "quadrillion", "quintillion"];

    if n == 0 {
        return "zero".to_string();
    }

    let mut segments: [Option<Segment>; 1 + NUM_SCALES] = Default::default();
    let mut segments_cursor = 0;

    while n != 0 {
        let right_three = n % 1000;

        if right_three > 0 {
            *segments
                .get_mut(segments_cursor)
                .expect("Segments cursor is out-of-bounds")
            
            =
            
            Some(Segment {
                written: Written::from(right_three),
                scale: segments_cursor.checked_sub(1).and_then(|cursor| SCALES.get(cursor).copied())
            });
        }

        n /= 1000;
        segments_cursor += 1;
    }

    let mut encoded = String::with_capacity(256);

    for cursor in (0..segments_cursor).rev() {
        let segment = segments
            .get_mut(cursor)
            .expect("Encoding cursor is out-of-bounds")
            .as_ref();

        if segment.is_none() {
            continue;
        }

        let segment = segment.unwrap();
        let written = &segment.written;

        if let Some(hundreds) = written.hundreds {
            encoded += hundreds;
            encoded += " hundred";

            if written.tens.is_some() || written.ones.is_some() {
                encoded += " ";
            }
        }

        if let Some(tens) = written.tens {
            encoded += tens;

            if written.ones.is_some() {
                encoded += "-";
            }
        }

        if let Some(ones) = written.ones {
            encoded += ones;
        }

        if let Some(scale) = segment.scale {
            encoded += " ";
            encoded += scale;
        }

        encoded += " ";
    }

    encoded.pop();
    encoded
}
