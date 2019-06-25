#![warn(clippy::pedantic)]

fn get_expected_closing_bracket(opening_bracket: char) -> char {
    match opening_bracket {
        '{' => '}',
        '[' => ']',
        '(' => ')',
        _ => panic!("unexpected opening bracket: \"{}\"", opening_bracket),
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut expected_closing_brackets = vec![];

    for c in string.chars() {
        match c {
            '{' | '[' | '(' => expected_closing_brackets.push(get_expected_closing_bracket(c)),

            '}' | ']' | ')' => {
                let expected_closing_bracket = expected_closing_brackets.pop();

                if expected_closing_bracket.is_none() || c != expected_closing_bracket.unwrap() {
                    return false;
                }
            }

            _ => (),
        };
    }

    expected_closing_brackets.is_empty()
}
