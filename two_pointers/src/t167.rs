use crate::Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut last_num_id = numbers.len() - 1;

        for (now_id, now_num) in numbers.iter().enumerate() {
            while now_num + numbers[last_num_id] > target {
                last_num_id -= 1;
            }

            if now_num + numbers[last_num_id] == target {
                return vec![(now_id + 1) as i32, (last_num_id + 1) as i32]
            }
        }
        unreachable!()
    }
}
