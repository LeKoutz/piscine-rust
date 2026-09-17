pub fn insertion_sort(slice: &mut[i32], steps: usize) {
    for i in 1..slice.len() {
        if i > steps {
            break;
        }
        let mut j = i;
        while j > 0 && slice[j] < slice[j-1] {
            slice.swap(j, j - 1);
            j -= 1;
        }
    }
}