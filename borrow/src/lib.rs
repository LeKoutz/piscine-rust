pub fn str_len(s: &str) -> usize {
    let len = s.chars().count();
    return len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "olá!";
        let result = str_len(&s);
        assert_eq!(result, 4);
    }
}
