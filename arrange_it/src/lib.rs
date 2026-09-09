pub fn arrange_phrase(phrase: &str) -> String {
    let word_count = phrase.split_whitespace().count();
    let mut result = vec![String::new(); word_count];
    for word in phrase.split_whitespace() {
        let mut stripped = String::new(); 
        let mut position: u32 = 0;
        for c in word.chars() {
            if c.is_numeric() {
                position = c.to_digit(10).unwrap();
            } else {
                stripped.push(c);
            }
        }
        result[position as usize -1] = String::from(stripped);
    }
    result.join(" ")
}
