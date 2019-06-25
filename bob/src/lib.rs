#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_yelling = {
        let letters: Vec<_> = message.chars().filter(|c| c.is_alphabetic()).collect();
        !letters.is_empty() && letters.into_iter().all(char::is_uppercase)
    };

    match (message.ends_with('?'), is_yelling) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        (false, false) => "Whatever.",
    }
}
