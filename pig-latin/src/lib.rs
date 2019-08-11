pub fn translate(input: &str) -> String {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    
    let input = input.to_string();

    let first_letter = input.chars().nth(0);

    if first_letter.is_none() {
        return input;
    }

    let first_letter = first_letter.unwrap();

    let is_first_letter_a_vowel = VOWELS.contains(&first_letter);
    
    if is_first_letter_a_vowel {
        input + "ay"
    } else {
        // Get the characters up to first vowel.
        let index_of_last_consonant = input
            .chars()
            .enumerate()
            .take_while(|(_, c)| !VOWELS.contains(&c))
            .last()
            .map(|(i, _)| i)
            .unwrap();

        dbg!(&input, &index_of_last_consonant);
        format!("{}{}ay", &input[(1+index_of_last_consonant)..], &input[..=index_of_last_consonant])        
    }
}
