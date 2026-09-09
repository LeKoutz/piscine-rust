pub fn char_length(s: &str) -> usize {
    s.chars().count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ascii() {
        let s = "ascii";
        assert_eq!(char_length(s), 5);
    }

    #[test]
    fn test_emoji() {
        let s = "❤😍";
        assert_eq!(char_length(s), 2);
    }
    #[test]
    fn test_chinese_char() {
        let s = "形声字";
        assert_eq!(char_length(s), 3);
    }
}