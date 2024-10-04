use crate::Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::VecDeque;
        let k = k as usize;

        if k == 1 {
            return nums;
        } else if k == 2 {
            let mut res: Vec<i32> = Vec::with_capacity(nums.len() - k + 1);
            let mut pr_val = nums[0];

            for now_el in nums.iter().skip(1) {
                res.push(if *now_el > pr_val { *now_el } else { pr_val } );
                pr_val = *now_el;
            }

            return res;
        } else if k >= nums.len() {
            let max_el = nums.iter().max().unwrap_or(&0);
            return vec![*max_el; nums.len() - k + 1]
        }

        let mut res: Vec<i32> = Vec::with_capacity(nums.len() - k + 1);
        let mut dq: VecDeque<usize> = VecDeque::with_capacity(8);

        let mut now_max = i32::MIN;

        for (now_el_id, now_el) in nums.iter().enumerate().take(k) {
            if *now_el >= now_max {
                now_max = *now_el;
                dq.clear();
                dq.push_front(now_el_id);
            } else {
                while let Some(val) = dq.pop_back() {
                    if nums[val] > *now_el {
                        dq.push_back(val);
                        break
                    }
                }
                dq.push_back(now_el_id)
            }
        }
        res.push(now_max);

        for (now_el_id, now_el) in nums.iter().enumerate().skip(k) {
            while let Some(val) = dq.pop_front() {
                if now_el_id - val < k {
                    if nums[val] < *now_el {
                        res.push(*now_el);
                        dq.clear();
                        dq.push_front(now_el_id);
                    } else {
                        res.push(nums[val]);
                        dq.push_front(val);

                        while let Some(sub_val) = dq.pop_back() {
                            if now_el_id - val < k && nums[sub_val] >= *now_el {
                                dq.push_back(now_el_id);
                                dq.push_back(sub_val);
                                break
                            }
                        }
                    }
                    break
                }
            };
        }

        res
    }
}
