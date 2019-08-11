pub fn translate(input: &str) -> String {
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::borrow::Cow::Owned;

    fn translate_word(word: &str) -> String {
        lazy_static! {
            static ref RULE_1: Regex = Regex::new(r"^(?:[aeiou]|xr|yt)\w*$").unwrap();
            static ref RULE_2: Regex = Regex::new(r"^(?P<consonant>[^aeiou]+)(?P<rest>\w*)$").unwrap();
            static ref RULE_3: Regex = Regex::new(r"^(?P<consonant>[^aeiou]*)qu(?P<rest>\w*)$").unwrap();
            static ref RULE_4: Regex = Regex::new(r"^(?P<consonant>[^aeiou]+)y(?P<rest>\w*)$").unwrap();
        }

        let word = word.to_string();

        if RULE_1.is_match(&word) {
            return word + "ay";
        }

        if let Owned(replaced) = RULE_4.replace(&word, "y${rest}${consonant}ay") {
            replaced
        } else if let Owned(replaced) = RULE_3.replace(&word, "${rest}${consonant}quay") {
            replaced
        } else if let Owned(replaced) = RULE_2.replace(&word, "${rest}${consonant}ay") {
            replaced
        } else {
            unreachable!("Encountered an word that does not match any of the rules: {}", word);
        }
    }

    input
        .split_ascii_whitespace()
        .map(translate_word)
        .collect::<Vec<_>>()
        .join(" ")
}
