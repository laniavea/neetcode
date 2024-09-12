use crate::Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let hashed_values: HashSet<i32> = nums.into_iter().collect();

        let mut max_len = 0;

        for val in &hashed_values {
            if !hashed_values.contains(&(val-1)) {
                let mut big_counter = val + 1;
                while hashed_values.contains(&big_counter) {
                    big_counter += 1;
                }

                if max_len < (big_counter - val) {
                    max_len = big_counter - val
                }
            }
        }

        max_len

        //
        // use std::collections::HashMap;
        //
        // let mut hashed_values: HashMap<i32, bool> = HashMap::new();
        //
        // for val in &nums {
        //     hashed_values.insert(*val, false);
        // }
        //
        // let mut max_len = 0;
        //
        // for val in nums {
        //     let now_val = hashed_values.get_mut(&val).unwrap();
        //
        //     if *now_val { continue; }
        //     *now_val = true;
        //
        //     let mut big_counter = val + 1;
        //     let mut small_counter = val - 1;
        //
        //     while let Some(sec_val) = hashed_values.get_mut(&big_counter) {
        //         *sec_val = true;
        //         big_counter += 1
        //     }
        //
        //     while let Some(sec_val) = hashed_values.get_mut(&small_counter) {
        //         *sec_val = true;
        //         small_counter -= 1
        //     }
        //
        //     let now_len = (val - small_counter) + (big_counter - val) - 1; // -1 for small, -1 for big, +1 for
        //                                                         // self val
        //     if now_len > max_len {
        //         max_len = now_len
        //     }
        // }
        //
        // max_len
    }
}
