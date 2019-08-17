pub fn translate(input: &str) -> String {
    use regex::Regex;
    use std::borrow::Cow::Owned;

    fn translate_word(word: &str) -> String {
        // todo: Use lazy_static! to cache constructed regular expressions.
        const RULES_2_3_4: &str = r"^(y)?([^\saeiouqy]*)(qu)?(q)?(\w+)$";
        fn r(pattern: &str) -> Regex { Regex::new(pattern).unwrap() }

        if r(r"^(?:[aeiou]|xr|yt)\w*$").is_match(&word) {
            word.to_owned() + "ay"
        } else if let Owned(replaced) = r(RULES_2_3_4).replace(&word, "${5}${2}${3}${4}${1}ay") {
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
