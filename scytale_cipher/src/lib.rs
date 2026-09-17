pub fn scytale_cipher(message: &str, i: usize) -> String {
    if i == 0 {
        return String::new();
    }
    let chars: Vec<char> = message.chars().collect();
    let rows: Vec<&[char]> = chars.chunks(i).collect();
    let mut result = String::new();

    for col in 0..i {
        for row in &rows {
            let c = row.get(col).copied().unwrap_or(' ');
            result.push(c);
        }
    }

    result.trim_end().to_string()
}