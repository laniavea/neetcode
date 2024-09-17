use crate::Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 1 {
            return vec!["()".to_string()]
        } else if n == 2 {
            return vec!["(())".to_string(), "()()".to_string()]
        }

        let n = n as usize;
        let new_n = n - 1;
        let all_n = n * 2;
        let pred_n = all_n - 2;

        let all_string = ")".repeat(all_n);

        let mut res: Vec<String> = Vec::with_capacity(n * 2);
        let end_pos: Vec<usize> = (1..=n).map(|now_id| n*2 - 2 * (n-now_id) - 2).collect();
        let mut con_pos: Vec<usize> = (0..n).collect();

        let mut ac_br = n - 1;

        loop {
            if ac_br != new_n {
                if con_pos[ac_br] != end_pos[ac_br] {
                    con_pos[ac_br] += 1;

                    con_pos[ac_br + 1] = if ac_br == new_n - 1 {
                        con_pos[ac_br] + 1
                    } else {
                        con_pos[ac_br]
                    };

                    ac_br += 1;
                } else {
                    ac_br -= 1;
                    if ac_br == 0 {
                        break
                    }
                }
            } else {
                let mut now_st = all_string.as_bytes().to_vec();

                for ch_br in con_pos.iter() {
                    now_st[*ch_br] -= 1;
                }

                res.push(String::from_utf8(now_st).unwrap());

                if con_pos[new_n] == pred_n {
                    ac_br -= 1;
                } else {
                    con_pos[ac_br] += 1
                }
            }
        }

        res
    }
}
