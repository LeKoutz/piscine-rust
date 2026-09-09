pub fn is_armstrong_number(nb: u32) -> Option<u32> {
    let digits: Vec<u32> = nb
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let count = digits.len() as u32;
    let sum: u32 = digits.iter().map(|d| d.pow(count)).sum();
    if sum == nb {
        Some(nb)
    } else {
        None
    }
}