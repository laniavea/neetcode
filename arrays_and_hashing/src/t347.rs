use crate::Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut values_index: HashMap<i32, u32> = HashMap::new();

        nums.iter().for_each(|el| *values_index.entry(*el).or_insert(0) += 1);

        let mut all_result: Vec<(u32, i32)> = values_index.iter().map(|data| (*data.1, *data.0)).collect();

        all_result.sort_by_key(|data| data.0);

        all_result.iter().rev().map(|data| data.1).take(k as usize).collect()
    }
}
