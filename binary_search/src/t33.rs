use crate::Solution;

impl Solution {
    pub fn search_t33(nums: Vec<i32>, target: i32) -> i32 {
        let mut left_bound = 0;
        let mut right_bound = nums.len() - 1;
        let most_left_el = nums[0];

        if most_left_el <= nums[right_bound] {
            match nums.binary_search(&target) {
                Ok(val) => return val as i32,
                Err(_) => return -1,
            }
        }

        if most_left_el == target { return 0 };

        if target < most_left_el {
            while left_bound <= right_bound {
                let now_id = (right_bound + left_bound) / 2;
                let now_el = nums[now_id];

                if now_el >= most_left_el {
                    left_bound = now_id + 1;
                    continue;
                }

                match now_el.cmp(&target) {
                    std::cmp::Ordering::Less => {
                        left_bound = now_id + 1;
                    },
                    std::cmp::Ordering::Greater => {
                        if now_id == 0 { return -1 }
                        right_bound = now_id - 1;
                    },
                    std::cmp::Ordering::Equal => return now_id as i32,
                };
            }

        } else {
            while left_bound <= right_bound {
                let now_id = (right_bound + left_bound) / 2;
                let now_el = nums[now_id];

                if now_el <= most_left_el {
                    if now_id == 0 {
                        if nums[1] == target { return 1 }
                        return -1
                    }
                    right_bound = now_id - 1;
                    continue;
                }

                match now_el.cmp(&target) {
                    std::cmp::Ordering::Less => {
                        left_bound = now_id + 1;
                    },
                    std::cmp::Ordering::Greater => {
                        if now_id == 0 {
                            return -1
                        }
                        right_bound = now_id - 1;
                    },
                    std::cmp::Ordering::Equal => return now_id as i32,
                };
            }
        }

        -1
    }
}
