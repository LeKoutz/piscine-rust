pub fn invert_sentence(string: &str) -> String {
    let mut tokens: Vec<(bool, String)> = Vec::new();
    let mut current = String::new();
    let mut current_is_word: Option<bool> = None;

    for c in string.chars() {
        let is_word_char = !c.is_whitespace();
        match current_is_word {
            Some(prev) if prev == is_word_char => {
                current.push(c);
            }
            _ => {
                if !current.is_empty() {
                    tokens.push((current_is_word.unwrap(), current.clone()));
                    current.clear();
                }
                current.push(c);
                current_is_word = Some(is_word_char);
            }
        }
    }
    if !current.is_empty() {
        tokens.push((current_is_word.unwrap(), current));
    }

    let mut words: Vec<String> = tokens
        .iter()
        .filter(|(is_word, _)| *is_word)
        .map(|(_, s)| s.clone())
        .collect();
    words.reverse();

    let mut word_iter = words.into_iter();
    let mut result = String::new();
    for (is_word, text) in &tokens {
        if *is_word {
            result.push_str(&word_iter.next().unwrap());
        } else {
            result.push_str(text);
        }
    }
    result
}