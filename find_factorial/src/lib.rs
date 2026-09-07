pub fn factorial(num: u64) -> u64 {
    if num == 0 { 1 } else { num * factorial(num - 1) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = factorial(6);
        assert_eq!(result, 720);
    }
}
