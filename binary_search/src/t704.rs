use crate::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut lb = 0;
        let mut rb = nums.len();

        while lb < rb {
            let now_id = lb + (rb - lb) / 2;
            match target.cmp(&nums[now_id]) {
                std::cmp::Ordering::Less => {
                    rb = now_id;
                },
                std::cmp::Ordering::Greater => {
                    lb = now_id + 1;
                },
                std::cmp::Ordering::Equal => {
                    return now_id as i32
                }
            }
        }

        -1
    }
}
