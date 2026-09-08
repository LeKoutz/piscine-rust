pub fn first_subword(s: String) -> String {
    let mut index = s.len();
    let new = s;
    for (i, c) in new.char_indices() {
        if i != 0 && c.is_ascii_uppercase() || c == '_'  {
            index = i;
            break;
        }
    }
    new[..index].to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camel_case_works() {
        let s1 = "CamelCase";
        let result = first_subword(s1.to_string());
        assert_eq!(result, "Camel");
    }
    #[test]
    fn pascal_case_works() {
        let s2 = "PascalCase";
        let result = first_subword(s2.to_string());
        assert_eq!(result, "Pascal");
    }
    #[test]
    fn snake_case_works() {
        let s3 = "snake_case";
        let result = first_subword(s3.to_string());
        assert_eq!(result, "snake");
    }
}
