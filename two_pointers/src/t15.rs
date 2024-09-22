use crate::Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        let all_len = sorted_nums.len();
        let mut res: Vec<Vec<i32>> = Vec::with_capacity(16);

        let mut pr_fr = sorted_nums[0] - 1;

        for (now_f_id, now_f) in sorted_nums.iter().enumerate() {

            if *now_f == pr_fr { continue; }
            if *now_f > 0 { break; }
            pr_fr = *now_f;

            let mut now_s_id = now_f_id + 1;
            let mut now_t_id = all_len - 1;
            
            while now_s_id < now_t_id {
                let target  = -(*now_f + sorted_nums[now_s_id]);
                if target < 0 { break }
                while now_s_id < now_t_id {
                    match target.cmp(&sorted_nums[now_t_id]) {
                        std::cmp::Ordering::Greater => {
                            break
                        }

                        std::cmp::Ordering::Equal => {
                            res.push(vec![*now_f, sorted_nums[now_s_id], sorted_nums[now_t_id]]);
                            break
                        }
                        _ => ()
                    }
                    now_t_id -= 1;

                    if sorted_nums[now_t_id] < 0 { break }
                }

                let pr_s = sorted_nums[now_s_id];
                now_s_id += 1;
                while sorted_nums[now_s_id] == pr_s && now_s_id < now_t_id {
                    now_s_id += 1;
                }
            }
        }

        res
    }
}
