pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut initials: Vec<String> = Vec::with_capacity(names.len());
    for name in names {
        let word_count = name.split_whitespace().count();
        let mut result = String::with_capacity(word_count * 3 - 1);
        for (i, word) in name.split_whitespace().enumerate() {
            if let Some(c) = word.chars().next() {
                result.push(c);
                result.push('.');
                if i != word_count - 1 {
                    result.push(' ');
                }
            }
        }
        initials.push(result);
    }
    initials
}