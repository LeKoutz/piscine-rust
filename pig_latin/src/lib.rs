pub fn pig_latin(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    match chars.as_slice() {
        [c, ..] if is_vowel(*c) => format!("{}ay", text),
        [c1, 'q', 'u', rest @ ..] if !is_vowel(*c1) => {
            format!("{}{}quay", rest.iter().collect::<String>(), c1)
        }
        _ => {
            let frst_cons_psn = chars.iter().position(|c| is_vowel(*c)).unwrap_or(chars.len());
            let (consonants, remainder) = chars.split_at(frst_cons_psn);
            format!("{}{}ay", remainder.iter().collect::<String>(), consonants.iter().collect::<String>())
        }
    }
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}