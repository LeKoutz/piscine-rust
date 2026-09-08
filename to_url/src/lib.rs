pub fn to_url(s: &str) -> String {
    let new: String = s.replace(" ", "%20");
    return new;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "Hello, world!";
        let result = to_url(&s);
        assert_eq!(result, "Hello,%20world!");
    }
}
