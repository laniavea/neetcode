mod t121;
mod t3;
mod t424;
mod t567;
mod t76;
mod t239;

pub struct Solution {}

fn main() {
    // t121
    // https://leetcode.com/problems/best-time-to-buy-and-sell-stock/description/
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);

    // t3
    // https://leetcode.com/problems/longest-substring-without-repeating-characters/description/
    assert_eq!(Solution::length_of_longest_substring(String::from("abcabcbb")), 3);
    assert_eq!(Solution::length_of_longest_substring(String::from("bbbbb")), 1);
    assert_eq!(Solution::length_of_longest_substring(String::from("pwwkew")), 3);
    assert_eq!(Solution::length_of_longest_substring(String::from("dvdf")), 3);

    // t424
    // https://leetcode.com/problems/longest-repeating-character-replacement/description/
    assert_eq!(Solution::character_replacement(String::from("ABAB"), 2), 4);
    assert_eq!(Solution::character_replacement(String::from("AABABBA"), 1), 4);
    assert_eq!(Solution::character_replacement(String::from("ABBB"), 2), 4);

    // t567
    // https://leetcode.com/problems/permutation-in-string/description/
    assert!(Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()));
    assert!(!Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string()));
    assert!(Solution::check_inclusion("abc".to_string(), "bbbca".to_string()));

    // t76
    // https://leetcode.com/problems/minimum-window-substring/description/
    assert_eq!(Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()), "BANC".to_string());
    assert_eq!(Solution::min_window("a".to_string(), "a".to_string()), "a".to_string());
    assert_eq!(Solution::min_window("a".to_string(), "aa".to_string()), "".to_string());
    assert_eq!(Solution::min_window("ab".to_string(), "b".to_string()), "b".to_string());
    assert_eq!(Solution::min_window("abcabdebac".to_string(), "cda".to_string()), "cabd".to_string());
    assert_eq!(Solution::min_window("BbBABaABbbb".to_string(), "aA".to_string()), "aA".to_string());
    assert_eq!(Solution::min_window("cfdfdasfhdaskjfdahsfka".to_string(), "z".to_string()), "".to_string());

    // t239
    // https://leetcode.com/problems/sliding-window-maximum/description/
    assert_eq!(Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3), [3, 3, 5, 5, 6, 7]);
    assert_eq!(Solution::max_sliding_window(vec![1], 1), [1]);
    assert_eq!(Solution::max_sliding_window(vec![1, 3, 1, 2, 0, 5], 3), [3, 3, 2, 5]);
    assert_eq!(Solution::max_sliding_window(vec![-7, -8, 7, 5, 7, 1, 6, 0], 4), [7, 7, 7, 7, 7]);
}
