pub struct WordProblem;

enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Op {
    fn from(s: &str) -> Option<Self> {
        use Op::*;
        Some(match s {
            "plus" => Add,
            "minus" => Subtract,
            "multiplied" => Multiply,
            "divided" => Divide,
            _ => return None,
        })
    }

    fn calc(&self, a: i32, b: i32) -> i32 {
        use Op::*;
        match self {
            Add => a + b,
            Subtract => a - b,
            Multiply => a * b,
            Divide => a / b,
        }
    }
}

pub fn answer(command: &str) -> Option<i32> {
    let mut number = None;
    let mut op: Option<Op> = None;

    for token in command.split(' ') {
        if let Some(o) = &mut op {
            if Op::from(token).is_some() {
                return None;
            }

            if let Ok(m) = token.trim_end_matches('?').parse::<i32>() {
                number = Some(o.calc(number.unwrap(), m));
                op = None;
            }
        }
        else if number.is_some() {
            if token.ends_with('?') {
                return None;
            }
            op = Op::from(token);
        } else if let Ok(n) = token.trim_end_matches('?').parse::<i32>() {
            number = Some(n);
        } else {
            continue;
        }
    }

    if op.is_some() {
        None
    } else {
        number
    }
}
