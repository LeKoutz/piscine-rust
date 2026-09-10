pub fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len()-1 {
        let mut swapped = false;
        for j in 0..arr.len()-i-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
                swapped = true;
            }
        }
        if !swapped { break }
    }
}