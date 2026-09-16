pub fn rotate(input: &str, key: i8) -> String {
    let mut result = String::new();
    for c in input.chars() {
        match c {
            letter @ ('a'..='z' | 'A'..='Z') => {
                let base = if letter.is_ascii_lowercase() {'a'} else {'A'};
                result.push(shift(letter, base, key))
            }
            _ => result.push(c),
        }
    }
    result
}

fn shift(c: char, base: char, key: i8) -> char {
    let position = c as u8 - base as u8;
    let shifted = (position as i32 + key as i32).rem_euclid(26);
    (base as u8 + shifted as u8) as char
}