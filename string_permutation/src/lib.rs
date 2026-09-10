use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() { return false }
    let mut map1: HashMap<char, u32> = HashMap::new();
    let mut map2: HashMap<char, u32> = HashMap::new();
    for c in s1.chars() {
        *map1.entry(c).or_insert(0) += 1;
    }
    for c in s2.chars() {
        *map2.entry(c).or_insert(0) += 1;
    }
    map1 == map2
}