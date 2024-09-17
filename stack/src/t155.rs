pub struct MinStack {
    data: Vec<i32>,
    min_data: Vec<usize>,
    now_min: i32,
    now_min_id: usize,
}

impl Default for MinStack {
    fn default() -> MinStack {
        MinStack::new()
    }
}

impl MinStack {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(8),
            min_data: Vec::with_capacity(4),
            now_min: i32::MAX,
            now_min_id: 0,
        }
    }

    pub fn push(&mut self, val: i32) {
        if val < self.now_min {
            self.min_data.push(self.now_min_id);
            self.now_min_id = self.data.len();
            self.now_min = val;
        }
        self.data.push(val)
    }

    pub fn pop(&mut self) {
        if let Some(popped_val) = self.data.pop() {
            if popped_val == self.now_min && self.data.len() <= self.now_min_id {

                if self.data.is_empty() {
                    self.now_min = i32::MAX;
                    self.min_data.pop();
                    return
                }

                let last_min_id = self.min_data.pop().unwrap();
                self.now_min_id = last_min_id;
                self.now_min = self.data[last_min_id]
            }
        }
    }

    pub fn top(&self) -> i32 {
        if self.data.is_empty() {
            return 0
        }
        self.data[self.data.len() - 1]
    }

    pub fn get_min(&self) -> i32 {
        self.now_min
    }
}
