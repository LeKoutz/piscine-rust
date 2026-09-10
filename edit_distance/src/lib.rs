/// Calculates the minimum number of changes (insertions, deletions and/or substitutions) 
/// which are needed to transform the `source` string to the `target` string
/// using Levenshtein's distance method.
pub fn edit_distance(source: &str, target: &str) -> usize {
    let source: Vec<char> = source.chars().collect();
    let target: Vec<char> = target.chars().collect();
    let n = source.len();
    let m = target.len();

    let mut table = vec![vec![0usize; m + 1]; n + 1];

    for i in 0..=n {
        table[i][0] = i;
    }
    for j in 0..=m {
        table[0][j] = j;
    }

    for i in 1..=n {
        for j in 1..=m {
            if source[i - 1] == target[j - 1] {
                table[i][j] = table[i - 1][j - 1];
            } else {
                let substitute = table[i - 1][j - 1];
                let delete = table[i - 1][j];
                let insert = table[i][j - 1];
                table[i][j] = 1 + substitute.min(delete).min(insert);
            }
        }
    }

    table[n][m]
}