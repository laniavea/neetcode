use crate::Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut pos_sp: Vec<(i32, f32)> = Vec::with_capacity(position.len());

        for (now_speed, now_pos) in speed.iter().zip(position) {
            pos_sp.push((now_pos, (target - now_pos) as f32 / *now_speed as f32))
        }

        pos_sp.sort_by_key(|val | val.0);

        let mut car_fleet: i32 = 0;
        let mut last_time: f32 = 0.0;

        for (_, now_time) in pos_sp.iter().rev() {
            if *now_time > last_time {
                last_time = *now_time;
                car_fleet += 1;
            }
        }

        car_fleet
    }
}
