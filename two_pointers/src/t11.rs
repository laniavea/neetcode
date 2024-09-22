use crate::Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left_st: Vec<(i32, i32)> = Vec::with_capacity(16);
        let mut max_left_el = height[0];

        left_st.push((0, height[0]));
        for (now_id, now_height) in height.iter().enumerate().skip(1) {
            if *now_height > max_left_el {
                left_st.push((now_id as i32, *now_height));
                max_left_el = *now_height;
            }
        }

        let all_len = height.len() - 1;
        let mut rigth_st: Vec<(i32, i32)> = Vec::with_capacity(16);

        rigth_st.push((all_len as i32, height[all_len]));
        let mut max_right_st = rigth_st[0].1;

        for (now_id, now_height) in height[left_st.last().unwrap().0 as usize..].iter().rev().enumerate().skip(1) {
            if *now_height > max_right_st {
                rigth_st.push(((all_len - now_id) as i32, *now_height));
                max_right_st = *now_height;
                if max_right_st == max_left_el { break; }
            }
        }

        let mut res: i32 = 0;
        let mut end_point = 0;

        for (l_id, l_h) in left_st {
            loop {
                let (r_id, r_h) = rigth_st[end_point];

                if l_h > r_h {
                    if r_h * (r_id - l_id) > res { res = r_h * (r_id - l_id) }
                    if end_point + 1 == rigth_st.len() { break; }
                    end_point += 1;

                } else {
                    if l_h * (r_id - l_id) > res { res = l_h * (r_id - l_id) }
                    break
                }
            }
        }

        res
    }
}
