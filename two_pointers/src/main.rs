mod t125;
mod t167;
mod t15;
mod t11;
mod t42;

pub struct Solution {}

fn main() {
    // t125
    // https://leetcode.com/problems/valid-palindrome/description/
    assert!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()));
    assert!(!Solution::is_palindrome("race a car".to_string()));
    assert!(Solution::is_palindrome(" ".to_string()));
    assert!(!Solution::is_palindrome("0P".to_string()));
    assert!(Solution::is_palindrome("a.".to_string()));

    // t167
    // https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/description/
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);

    // t15
    // https://leetcode.com/problems/3sum/description/
    assert_eq!(Solution::three_sum(vec![-1, 0, 1, 2, -1, 4]), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    assert_eq!(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    assert_eq!(Solution::three_sum(vec![1, 1, -2]), vec![vec![-2, 1, 1]]);
    assert!(Solution::three_sum(vec![1, 2, -2, -1]).is_empty());
    assert_eq!(Solution::three_sum(vec![1, -1, -1, 0]), vec![vec![-1, 0, 1]]);
    assert!(Solution::three_sum(vec![-4, -2, -1]).is_empty());

    // t11
    // https://leetcode.com/problems/container-with-most-water/description/
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(Solution::max_area(vec![1, 1]), 1);

    // t42
    // https://leetcode.com/problems/trapping-rain-water/description/
    assert_eq!(Solution::trap_orig_stack(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(Solution::trap_orig_stack(vec![4, 2, 0, 3, 2, 5]), 9);

    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
}
