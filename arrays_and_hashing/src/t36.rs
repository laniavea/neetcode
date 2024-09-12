use crate::Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;
        let mut columns: Vec<HashSet<char>> = vec![HashSet::with_capacity(9); 9];
        let mut cubes: Vec<HashSet<char>> = vec![HashSet::with_capacity(9); 3];
        let mut rows: HashSet<char> = HashSet::with_capacity(9);

        for (row_id, now_row) in board.iter().enumerate() {
            rows.clear();

            for (col_id, now_char) in now_row.iter().enumerate().filter(|now_char| *now_char.1 != '.') {
                if rows.contains(now_char) || cubes[col_id / 3].contains(now_char) || columns[col_id].contains(now_char) {
                    return false
                }

                rows.insert(*now_char);
                columns[col_id].insert(*now_char);
                cubes[col_id / 3].insert(*now_char);
            }

            if row_id % 3 == 2 && row_id != 8 {
                cubes.iter_mut().for_each(|hs| hs.clear())
            }
        }

        true
    }
}
