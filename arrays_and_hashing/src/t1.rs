use crate::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut differences: HashMap<i32, i32> = HashMap::with_capacity(nums.len() / 2 + 1);

        for (now_id, now_num) in nums.iter().enumerate() {
            match differences.get(&(target - now_num)) {
                Some(key) => return vec![*key, now_id as i32],
                None => differences.insert(*now_num, now_id as i32),
            };
        }
        unreachable!();
    }
}
