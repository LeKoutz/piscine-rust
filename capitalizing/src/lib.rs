pub fn capitalize_first(input: &str) -> String {
    if input == "" { return String::new() }
    format!("{}{}", input.chars().next().unwrap().to_ascii_uppercase(), &input[1..])
}

pub fn title_case(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut capitalize_next = true;
    for c in input.chars() {
        if c.is_whitespace() {
            result.push(c);
            capitalize_next = true;
        } else if capitalize_next {
            let ch = c.to_ascii_uppercase();
            result.push(ch);
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| if c.is_ascii_uppercase() {
            c.to_ascii_lowercase() 
        } else {
            c.to_ascii_uppercase()
        })
        .collect()
}