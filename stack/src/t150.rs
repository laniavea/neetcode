use crate::Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut digits_stack: Vec<i32> = Vec::with_capacity(8);

        for now_token in tokens {
            match now_token.as_str() {
                "*" => {
                    let first_val = digits_stack.pop().unwrap();
                    let sec_val = digits_stack.pop().unwrap();
                    digits_stack.push(first_val * sec_val)
                },
                "/" => {
                    let sec_val = digits_stack.pop().unwrap();
                    let first_val = digits_stack.pop().unwrap();
                    digits_stack.push(first_val / sec_val)
                },
                "+" => {
                    let first_val = digits_stack.pop().unwrap();
                    let sec_val = digits_stack.pop().unwrap();
                    digits_stack.push(first_val + sec_val)
                },
                "-" => {
                    let sec_val = digits_stack.pop().unwrap();
                    let first_val = digits_stack.pop().unwrap();
                    digits_stack.push(first_val - sec_val)
                },
                _ => {
                    digits_stack.push(now_token.parse().unwrap())
                }
            }
        }

        digits_stack[0]
    }
}
