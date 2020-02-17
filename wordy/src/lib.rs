pub fn answer(command: &str) -> Option<i32> {
    let mut number = None;
    let mut op: Option<Op> = None;

    for token in command.split(' ') {
        if let Some(o) = &op {
            // Look for second operand.

            if Op::from(token).is_some() {
                // Found another operation in the middle of an operation.
                // So the input is malformed.
                return None;
            }

            if let Some(m) = parse(token) {
                // Found second operand. Update accumulator.
                number = Some(o.calc(number.unwrap(), m));

                // Reset operation so we can begin a new operation.
                op = None;
            }
        }
        else if number.is_some() {
            // Look for operation.

            if token != "power?" && token.ends_with('?') {
                // Input ended before operating on accumulator.
                // So the input is malformed.
                return None;
            }
            op = Op::from(token);
        } else {
            // Look for first operand.
            number = parse(token);
        }
    }

    if op.is_some() {
        // Input ended before completing current operation.
        // So the input is malformed.
        None
    } else {
        number
    }
}

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