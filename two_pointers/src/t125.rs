use crate::Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut backward_sym_iter = s.bytes().rev();

        let mut now_f_sym;

        for forward_sym in s.bytes() {
            match forward_sym {
                65..=90 => { now_f_sym = forward_sym + 32 },
                97..=122 | 48..=57 => { now_f_sym = forward_sym },
                _ => continue,
            }

            for backward_sym in backward_sym_iter.by_ref() {
                match backward_sym {
                    65..=90 => {
                        if backward_sym + 32 != now_f_sym {
                            return false
                        }
                        break
                    },
                    97..=122 | 48..=57 => {
                        if backward_sym != now_f_sym {
                            return false
                        }
                        break
                    },
                    _ => (),
                }
            }

        }

        true
    }
}
