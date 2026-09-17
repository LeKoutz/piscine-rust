use std::cmp::Ordering;

pub fn binary_search(sorted_list: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = sorted_list.len();

    while low < high {
        let mid = low + (high - low) / 2;
        match sorted_list[mid].cmp(&target) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid,
        }
    }
    None
}