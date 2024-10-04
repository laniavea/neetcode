use crate::Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let k = k as usize;

        let mut res = 0;
        let mut alp_c: [usize; 26] = [0; 26];

        let mut left_br = 0;
        let mut left_s = s.bytes();
        let mut max = 0;

        for (right_br, now_s) in s.bytes().enumerate() {
            alp_c[now_s as usize - 65] += 1;
            max = max.max(alp_c[now_s as usize - 65]);

            if right_br - left_br + 1 - max > k {
                alp_c[left_s.next().unwrap() as usize - 65] -= 1;
                left_br += 1;
            }

            res = res.max(right_br - left_br + 1)
        }

        res as i32
    }
}
