use crate::Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut heights_st: Vec<(i32, i32)> = Vec::with_capacity(32);

        let mut largest_val = 0;
        let mut latest_val = heights[0];

        heights_st.push((0, 0));
        heights_st.push((heights[0], 1));

        for now_height in heights.iter().skip(1) {
            if *now_height > latest_val {
                heights_st.push((*now_height, 1));
                latest_val = *now_height
            } else {
                let mut temp_h = 0;
                loop {
                    let (st_h_val, st_h_num) = heights_st.pop().unwrap();
                    temp_h += st_h_num;

                    match st_h_val.cmp(now_height) {
                        std::cmp::Ordering::Greater => { largest_val = (st_h_val * temp_h).max(largest_val); },
                        std::cmp::Ordering::Equal => {
                            heights_st.push((st_h_val, temp_h + 1));
                            break
                        },
                        std::cmp::Ordering::Less => {
                            heights_st.push((st_h_val, st_h_num));
                            heights_st.push((*now_height, temp_h - st_h_num + 1));
                            break
                        }
                    }
                }
                latest_val = *now_height
            }
        }

        let mut temp_h = 0;
        for (now_h, now_c) in heights_st.iter().rev() {
            temp_h += now_c;
            largest_val = largest_val.max(now_h * temp_h);
        }

        largest_val
    }
}
