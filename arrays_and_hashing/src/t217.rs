use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut used_nums: HashSet<i32> = HashSet::with_capacity(nums.len());

        used_nums.insert(nums[0]);

        for value in nums[1..].iter() {
            if used_nums.contains(value) {
                return true
            }
            used_nums.insert(*value);
        }
        false
    }
}
