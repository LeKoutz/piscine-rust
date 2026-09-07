pub fn rev_str(input: &str) -> String {
    input.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reverses_a_string() {
        assert_eq!(rev_str("hello"), "olleh");
    }
}
