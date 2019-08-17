pub fn translate(input: &str) -> String {
    use regex::Regex;
    use std::borrow::Cow::Owned;

    fn translate_word(word: &str) -> String {
        // todo: Use lazy_static! to cache constructed regular expressions.
        const RULE_1: &str = r"^(?:[aeiou]|xr|yt)\w*$";
        const RULES_2_3_4: &str = r"^(y)?([^\saeiouqy]*)(qu)?(q)?(\w+)$";
        const REPLACE: &str = "${5}${2}${3}${4}${1}ay";
        const R: fn(&str) -> Regex = |pattern| Regex::new(pattern).unwrap();

        if R(RULE_1).is_match(&word) {
            word.to_owned() + "ay"
        } else if let Owned(replaced) = R(RULES_2_3_4).replace(&word, REPLACE) {
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
