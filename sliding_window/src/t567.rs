use crate::Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut alp_mas: [i32; 26] = [0; 26];
        s1.bytes().for_each(|val| alp_mas[val as usize - 97] += 1);

        for now_sm in s2.bytes().take(s1.len()) {
            alp_mas[now_sm as usize - 97] -= 1;
        }

        let mut zeros_num = 0;
        alp_mas.iter().for_each(|el| if *el == 0 { zeros_num += 1 });
        if zeros_num == 26 { return true }

        let mut left_br = s2.bytes();

        for now_sm in s2.bytes().skip(s1.len()) {
            alp_mas[now_sm as usize - 97] -= 1;
            if alp_mas[now_sm as usize - 97] == 0 {
                zeros_num += 1;
            } else if alp_mas[now_sm as usize - 97] == -1 {
                zeros_num -= 1;
            }

            let t_val = left_br.next().unwrap() as usize - 97;
            alp_mas[t_val] += 1;

            if alp_mas[t_val] == 0 {
                zeros_num += 1;
                if zeros_num == 26 {
                    return true
                }
            } else if alp_mas[t_val] == 1 {
                zeros_num -= 1;
            }
        }

        false
    }
}
