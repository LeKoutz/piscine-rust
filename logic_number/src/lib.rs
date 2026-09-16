// Same solution as is_armstrong_number exercise
pub fn number_logic(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let count = digits.len() as u32;
    let sum: u32 = digits.iter().map(|d| d.pow(count)).sum();
    if sum == num {
        true
    } else {
        false
    }
}
