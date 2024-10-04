use crate::Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut sub_st: [u8; 128] = [0; 128];
        let mut all_mas: [bool; 128] = [false; 128];
        let mut res = 0;
        let mut cont: usize = 0;

        for now_s in s.bytes() {
            if all_mas[now_s as usize] {
                res = res.max(cont);
                let mut t_conn = 0;
                for (t_con, now_sub_st_s) in sub_st.iter().enumerate() {
                    all_mas[*now_sub_st_s as usize] = false;
                    if *now_sub_st_s == now_s {
                        t_conn = t_con + 1;
                        break
                    }
                }

                sub_st.rotate_left(t_conn);
                sub_st[128-t_conn..128].iter_mut().for_each(|val| *val = 0);
                cont -= t_conn;
            }

            all_mas[now_s as usize] = true;
            sub_st[cont] = now_s;
            cont += 1;
        }

        res.max(cont) as i32
    }
}
