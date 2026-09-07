pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

pub fn at_index(slice: &[String], index: usize) -> &str {
    &slice[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut groceries = Vec::new();
        insert(&mut groceries, String::from("nuts"));
        let result = at_index(&groceries, 0);
        assert_eq!(result, "nuts");
    }
}
