pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for (position, element) in array.iter().rev().enumerate() {
        if *element == key {
            return Some(array.len()-1-position);
        }
    }
    None
}
