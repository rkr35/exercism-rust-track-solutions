pub fn translate(input: &str) -> String {
    use regex::Regex;
    
    lazy_static::lazy_static! {
        static ref RULE_1: Regex = Regex::new(r"^(?:[aeiou]|xr|yt)\w*$").unwrap();
        static ref RULES_2_3_4: Regex = Regex::new(r"^(y)?([^aeiouqy]*)(qu)?(q)?(\w+)$").unwrap();
    }

    let mut s: String = input
        .split(' ')
        .map(|word| {
            use std::borrow::Cow::Owned;
            if RULE_1.is_match(&word) {
                word.to_owned() + "ay "
            } else if let Owned(replaced) = RULES_2_3_4.replace(&word, "${5}${2}${3}${4}${1}ay ") {
                replaced
            } else {
                unreachable!("Encountered a word that does not match any of the rules: \"{}\"", word)
            }
        })
        .collect();

    s.pop();
    s
}
