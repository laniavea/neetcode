use crate::Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false
        }

        let mut letters_arr = [0i16; 26];

        for (now_letter_s, now_letter_t) in s.bytes().zip(t.bytes()) {
            letters_arr[(now_letter_s - 97) as usize] += 1;
            letters_arr[(now_letter_t - 97) as usize] -= 1;
        }

        if letters_arr.iter().all(|x| *x == 0) {
            return true
        }

        false
    }
}
