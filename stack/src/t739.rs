use crate::Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let t_last = temperatures.len() - 1;

        let mut res: Vec<i32> = vec![0; t_last + 1];

        let mut all_data: Vec<(i32, u32)> = Vec::with_capacity(64);
        all_data.push((temperatures[t_last],0));

        let mut last_el: i32 = temperatures[t_last];
        let mut all_time_max: i32 = last_el;

        for (now_id, now_temp) in temperatures.iter().rev().enumerate().skip(1) {
            if *now_temp < last_el {
                res[t_last - now_id] = 1;
                all_data.push((*now_temp, now_id as u32));
                last_el = *now_temp;
            } else if *now_temp >= all_time_max {
                all_data = vec![(*now_temp, now_id as u32)];
                all_time_max = *now_temp
            } else {
                loop {
                    let last_data = all_data.pop().unwrap();
                    if *now_temp < last_data.0 {
                        res[t_last - now_id] = (now_id - last_data.1 as usize) as i32;

                        all_data.push(last_data);
                        all_data.push((*now_temp, now_id as u32));

                        last_el = *now_temp;

                        break
                    }
                }
            }
        }

        res
    }
}
