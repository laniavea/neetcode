use crate::Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left_bound = 0;
        let mut right_bound = nums.len() - 1;
        let most_left_el = nums[0];

        if most_left_el <= nums[right_bound] {
            return most_left_el
        }

        while left_bound <= right_bound {
            let now_id = (right_bound + left_bound) / 2;
            let now_el = nums[now_id];

            if now_el < most_left_el {
                right_bound = now_id - 1;
            } else {
                left_bound = now_id + 1;
            }
        }

        nums[left_bound]
    }
}
