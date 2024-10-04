use crate::Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.len() > s.len() { return String::new() };

        use std::collections::HashSet;
        let mut chars_to_find: HashSet<u8> = HashSet::new();
        let mut alph_ch = [0; 58];

        t.bytes().for_each(|el| {
            chars_to_find.insert(el);
            alph_ch[(el - b'A') as usize] += 1;
        });

        let mut left_val = 0;
        let mut left_br = 0;

        let mut min_res: usize = usize::MAX;
        let mut ids = (0, s.len()-1);

        let mut vals_num = chars_to_find.len();

        for (right_br, now_sm) in s.bytes().enumerate() {
            if chars_to_find.contains(&now_sm) {
                alph_ch[(now_sm - b'A') as usize] -= 1;

                if now_sm == left_val && vals_num == 0 {
                    for (el_id, el) in s.as_bytes()[left_br..].iter().enumerate() {
                        if chars_to_find.contains(el) {
                            if alph_ch[(el - b'A') as usize] == 0 {
                                left_br += el_id;
                                left_val = *el;
                                break 
                            };
                            alph_ch[(el - b'A') as usize] += 1;
                        }
                    }

                    if right_br - left_br < min_res {
                        min_res = right_br - left_br;
                        ids = (left_br, right_br);
                    }

                } else if alph_ch[(now_sm - b'A') as usize] == 0 { 
                    vals_num -= 1;
                    if vals_num == 0 {
                        for (el_id, el) in s.as_bytes()[left_br..].iter().enumerate() {
                            if chars_to_find.contains(el) {
                                if alph_ch[(el - b'A') as usize] == 0 {
                                    left_br += el_id;
                                    break 
                                };
                                alph_ch[(el - b'A') as usize] += 1;
                            }
                        }

                        left_val = s.as_bytes()[left_br];
                        min_res = right_br - left_br;
                        ids = (left_br, right_br);
                    }
                }
            }
        }

        if vals_num != 0 {
            return String::new()
        }
        s[ids.0..=ids.1].to_string()
    }
}
