use crate::Solution; 

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut target_row = matrix.len() - 1;
        for (now_row_id, now_row) in matrix.iter().enumerate() {
            if now_row[0] > target {
                if now_row_id == 0 { return false }
                target_row = now_row_id - 1;
                break
            }
        }

        matrix[target_row].binary_search(&target).is_ok()
    }
}
