pub fn is_luhn_formula(code: &str) -> bool {
    let mut digits = Vec::new();
    for c in code.chars() {
        match c {
            '0'..='9' => digits.push(c.to_digit(10).unwrap()),
            ' ' => continue,
            _ => return false,
        }
    }

    if digits.len() < 2 {
        return false;
    }

    let sum: u32 = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| {
            if i % 2 == 1 {
                match d * 2 {
                    doubled if doubled > 9 => doubled - 9,
                    doubled => doubled,
                }
            } else {
                d
            }
        })
        .sum();
    sum % 10 == 0
}