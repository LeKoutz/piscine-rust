const NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn solve_board(minefield: &[&str]) -> Vec<String> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    for &row in minefield {
        let row: Vec<char> = row.chars().collect();
        grid.push(row);
    }
    let mut result: Vec<String> = Vec::new();
    for (row_idx, row) in grid.iter().enumerate() {
        let mut row_output = String::new();
        for (col_idx, col) in row.iter().enumerate() {
            if *col == '*' {
                row_output.push('*');
            } else {
                let mut mines_count = 0;
                for (dr, dc) in NEIGHBORS {
                    let new_row = row_idx as isize + dr;
                    let new_col = col_idx as isize + dc;
                    if new_row < 0 || new_row >= grid.len() as isize {
                        continue;
                    }
                    if new_col < 0 || new_col >= row.len() as isize {
                        continue;
                    }
                    if grid[new_row as usize][new_col as usize] == '*' {
                        mines_count += 1;
                    }
                }
                if mines_count == 0 {
                    row_output.push(' ');
                } else {
                    row_output.push(('0' as u8 + mines_count as u8) as char);
                }
            }
        }
        result.push(row_output);
    }
    result
}
