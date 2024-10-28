use std::collections::HashMap;

pub struct TimeMap {
    values: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    pub fn new() -> Self {
        Self {
            values: HashMap::with_capacity(16),
        }
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        if let Some(ent) = self.values.get_mut(&key) {
            ent.push((timestamp, value));
        } else {
            self.values.insert(key, vec![(timestamp, value)]);
        }
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(ent) = self.values.get(&key) {
            let mut lb = 0;
            let mut rb = ent.len() - 1;

            while lb < rb {
                let now_id = (rb + lb) / 2;
                match ent[now_id].0.cmp(&timestamp) {
                    std::cmp::Ordering::Greater => { rb = now_id},
                    std::cmp::Ordering::Less => { lb = now_id + 1},
                    std::cmp::Ordering::Equal => { return ent[now_id].1.clone() }
                }
            }

            if timestamp < ent[rb].0 {
                if rb == 0 { return String::new() }
                return ent[rb - 1].1.clone();
            } else {
                return ent[rb].1.clone();
            }
        }

        String::new()
    }
}
