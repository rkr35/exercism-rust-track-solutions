pub struct WordProblem;

pub fn answer(command: &str) -> Option<i32> {
    enum Op {
        Add,
        Subtract,
        Multiply,
        Divide,
    }

    use Op::*;

    let tokens = command.split(' ');
    let mut number = None;
    let mut op = None;

    for token in tokens {
        if let Some(o) = &mut op {
            match token {
                "plus" => return None,
                "minus" => return None,
                "multiplied" => return None,
                "divided" => return None,
                _ => (),
            };

            if let Ok(n) = token.trim_end_matches('?').parse::<i32>() {
                match o {
                    Add => number = Some(number.unwrap() + n),
                    Subtract => number = Some(number.unwrap() - n),
                    Multiply => number = Some(number.unwrap() * n),
                    Divide => number = Some(number.unwrap() / n),
                }
                op = None;
            }
        }
        else if number.is_some() {
            match token {
                "plus" => op = Some(Add),
                "minus" => op = Some(Subtract),
                "multiplied" => op = Some(Multiply),
                "divided" => op = Some(Divide),
                _ if token.ends_with('?') => return None,
                _ => (),
            }
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
