use crate::Solution;

impl Solution {
    pub fn trap_orig_stack(height: Vec<i32>) -> i32 {
        let mut res: i32 = 0;

        let mut left_borders: Vec<(usize, i32)> = vec![(0, 0)];
        let mut last_left_border: i32 = 0;

        for (now_id, now_height) in height.iter().enumerate() {
            if *now_height >= last_left_border {
                let (_, mut pr_left_border_h) = left_borders.pop().unwrap();

                while let Some((now_left_border_id, now_left_border_h)) = left_borders.pop() {
                    match now_left_border_h.cmp(now_height) {
                        std::cmp::Ordering::Less => {
                            res += (now_left_border_h - pr_left_border_h) * (now_id - now_left_border_id - 1) as i32;
                            pr_left_border_h = now_left_border_h;
                        },
                        std::cmp::Ordering::Equal => {
                            res += (now_left_border_h - pr_left_border_h) * (now_id - now_left_border_id - 1) as i32;
                            break;
                        },
                        std::cmp::Ordering::Greater => {
                            res += (*now_height - pr_left_border_h) * (now_id - now_left_border_id - 1) as i32;
                            left_borders.push((now_left_border_id, now_left_border_h));
                            break;
                        }
                    }
                }
            }

            last_left_border = *now_height;
            left_borders.push((now_id, *now_height));
        }

        res
    }

    pub fn trap(height: Vec<i32>) -> i32 {
        let mut res: i32 = 0;
        let mut left_p = 0;
        let mut right_p = height.len() - 1;

        while left_p < right_p {
            if height[left_p] < height[right_p] {
                let now_h = height[left_p];
                left_p += 1;
                while height[left_p] < now_h {
                    res += now_h - height[left_p];
                    left_p += 1;
                }
            } else {
                let now_h = height[right_p];
                right_p -= 1;
                while height[right_p] < now_h {
                    res += now_h - height[right_p];
                    right_p -= 1;
                }
            }
        }

        res
    }
}
