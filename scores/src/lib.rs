pub fn score(string: &str) -> u64 {
    let mut sum = 0;
    for c in string.chars() {
        match c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o'| 'u' | 'l'| 'n' | 'r' | 's' | 't' => sum += 1,
            'd' | 'g' => sum += 2,
            'b' | 'c' | 'm' | 'p' => sum += 3,
            'f' | 'h' | 'v' | 'w' | 'y' => sum += 4,
            'k' => sum += 5,
            'j' | 'x' => sum += 8,
            'q' | 'z' => sum += 10,
            _ => sum += 0,
        }
    }
    sum
}
