pub fn translate(input: &str) -> String {
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::borrow::Cow::Owned;

    fn translate_word(word: &str) -> String {
        lazy_static! {
            static ref RULE_1: Regex = Regex::new(r"^(?:[aeiou]|xr|yt)\w*$").unwrap();
            static ref RULES_2_3_4: Regex = Regex::new(r"^(y)?([^\saeiouqy]*)(qu)?(q)?(\w+)$").unwrap();
        }

        let word = word.to_string();

        if RULE_1.is_match(&word) {
            word + "ay"
        } else if let Owned(replaced) = RULES_2_3_4.replace(&word, "${5}${2}${3}${4}${1}ay") {
            replaced
        } else {
            unreachable!("Encountered a word that does not match any of the rules: \"{}\"", word);
        }
    }

    input
        .split_ascii_whitespace()
        .map(translate_word)
        .collect::<Vec<_>>()
        .join(" ")
}
