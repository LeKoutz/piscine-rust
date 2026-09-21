pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut result:Vec<Box<u32>> = Vec::new();
    let numbers = s.split_whitespace();
    for number in numbers {
        match number.ends_with("k") {
            false => {
                let parsed: f64 = number.parse().expect(&format!("Failed to parse number {}", number));
                let boxed = Box::new(parsed as u32);
                result.push(boxed);
            },
            true => {
                let number = number.strip_suffix("k").unwrap();
                let parsed: f64 = number.parse().expect(&format!("Failed to parse number {}", number));
                let mult = parsed * 1000.0;
                let boxed = Box::new(mult as u32);
                result.push(boxed);
            }
        }
    }
    result
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    for number in a {
        result.push(*number);
    }
    result
}
