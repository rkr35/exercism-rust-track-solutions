enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
}

impl Op {
    fn from(s: &str) -> Option<Self> {
        use Op::*;
        Some(match s {
            "plus" => Add,
            "minus" => Subtract,
            "multiplied" => Multiply,
            "divided" => Divide,
            "raised" => Exponent,
            _ => return None,
        })
    }

    fn calc(&self, a: i32, b: i32) -> i32 {
        use Op::*;
        use std::convert::TryInto;
        match self {
            Add => a + b,
            Subtract => a - b,
            Multiply => a * b,
            Divide => a / b,
            Exponent => a.pow(b.try_into().unwrap()),
        }
    }
}

fn parse(token: &str) -> Option<i32> {
    ["?", "st", "nd", "rd", "th"]
        .iter()
        .find_map(|s| token.trim_end_matches(s).parse().ok())
}

pub fn answer(command: &str) -> Option<i32> {
    let mut number = None;
    let mut op: Option<Op> = None;

    for token in command.split(' ') {
        if let Some(o) = &op {
            if Op::from(token).is_some() {
                return None;
            }

            if let Some(m) = parse(token) {
                number = Some(o.calc(number.unwrap(), m));
                op = None;
            }
        }
        else if number.is_some() {
            if token != "power?" && token.ends_with('?') {
                return None;
            }
            op = Op::from(token);
        } else {
            number = parse(token);
        }
    }

    if op.is_some() {
        None
    } else {
        number
    }
}
