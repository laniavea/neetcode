use crate::Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut max_pile = 0;
        let mut banana_sum: i64 = 0;
        for now_pile in &piles {
            let now_pile = *now_pile as i64;
            banana_sum += now_pile;
            if now_pile > max_pile {
                max_pile = now_pile;
            }
        }

        let mut lb = (banana_sum + h as i64- 1) / h as i64;
        let mut rb = max_pile;
        let mut res = max_pile as i32;

        while lb <= rb {
            let now_ban = ((rb - lb) / 2 + lb) as i32;
            let mut t_res = 0;

            for now_pile in &piles {
                t_res += (now_pile + now_ban - 1) / now_ban
            }
            if t_res <= h {
                res = now_ban;
                rb = now_ban as i64 - 1;
            } else {
                lb = now_ban as i64 + 1;
            }
        }

        res
    }
}
