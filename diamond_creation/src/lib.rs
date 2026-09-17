pub fn get_diamond(c: char) -> Vec<String> {
    let n = (c as u8 - 'A' as u8) as usize;
    let size = 2 * n + 1;
    let center = n;
    let mut rows = Vec::with_capacity(size);

    for i in 0..size {
        let row_offset = (i as isize - n as isize).unsigned_abs();
        let letter_index = n - row_offset;
        let letter = ('A' as u8 + letter_index as u8) as char;

        let mut row = vec![' '; size];
        match letter_index {
            0 => row[center] = letter,
            col_offset => {
                row[center - col_offset] = letter;
                row[center + col_offset] = letter;
            }
        }

        rows.push(row.into_iter().collect());
    }
    rows
}