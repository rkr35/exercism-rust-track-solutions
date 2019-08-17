pub fn translate(input: &str) -> String {
    use regex::Regex;
    
    lazy_static::lazy_static! {
        static ref RULES: Regex = Regex::new(r"(^(?:[aeiou]|xr|yt)\w*$)|^(y)?([^aeiouqy]*)(qu)?(q)?(\w+)$").unwrap();
    }

    let mut s: String = input
        .split(' ')
        .map(|word| {
            use std::borrow::Cow::Owned;
            if let Owned(replaced) = RULES.replace(&word, "${1}${6}${3}${4}${5}${2}ay ") {
                replaced
            } else {
                unreachable!("Encountered a word that does not match any of the rules: \"{}\"", word)
            }
        })
        .collect();

    s.pop();
    s
}
