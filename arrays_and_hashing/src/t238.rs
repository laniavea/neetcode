use crate::Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut now_max_id = nums.len();

        let mut res: Vec<i32> = Vec::with_capacity(now_max_id);

        let mut now_mul = 1;
        for value in &nums {
            res.push(now_mul);
            now_mul *= value;
        }

        now_mul = 1;

        for value in nums.into_iter().rev() {
            now_max_id -= 1;
            res[now_max_id] *= now_mul;
            now_mul *= value;
        }

        res
    }
}
