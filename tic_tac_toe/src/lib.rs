pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    const PLAYERS: [char; 2] = ['X', 'O'];
    for player in PLAYERS {
        if diagonals(player, table) || horizontal(player, table) || vertical(player, table) {
            return format!("player {player} won");
        }
    }
    return String::from("tie");
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    let diag = (0..3).all(|i| table[i][i] == player);
    let anti_diag = (0..3).all(|i| table[i][2-i] == player);
    diag || anti_diag
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for row in table.iter() {
        if row.iter().all(|x| *x == player) {
            return true
        }
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for col in 0..3 {
        let mut all_match = true;
        for row in 0..3 {
            if table[row][col] != player {
                all_match = false;
            }
        }
        if all_match {
            return true;
        }
    }
    false
}