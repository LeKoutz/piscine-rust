// Another valid solution is the string_permutation exercise, just add the lowercase part

pub fn is_anagram(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let mut chars1: Vec<char> = s1.to_ascii_lowercase().chars().collect();
    let mut chars2: Vec<char> = s2.to_ascii_lowercase().chars().collect();
    chars1.sort();
    chars2.sort();
    chars1 == chars2
}