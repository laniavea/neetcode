use crate::Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut char_stack: Vec<u8> = Vec::with_capacity(10);

        for now_let in s.bytes() {
            match now_let {
                40 => char_stack.push(41),
                91 => char_stack.push(93),
                123 => char_stack.push(125),
                _ => {
                    if char_stack.pop().unwrap_or(0) != now_let {
                        return false
                    }
                }
            }
        }

        if char_stack.is_empty() {
            return true
        }

        false
    }
}
