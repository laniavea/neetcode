use t981::TimeMap;

mod t704;
mod t74;
mod t875;
mod t153;
mod t33;
mod t981;
mod t4;

pub struct Solution {}

fn main() {
    // t704
    // https://leetcode.com/problems/binary-search/description/
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    assert_eq!(Solution::search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    assert_eq!(Solution::search(vec![5], -5), -1);

    // t74
    // https://leetcode.com/problems/search-a-2d-matrix/description/
    assert!(Solution::search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]], 3));
    assert!(!Solution::search_matrix(vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]], 13));

    // t875
    // https://leetcode.com/problems/koko-eating-bananas/description/
    assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
    assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
    assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
    assert_eq!(Solution::min_eating_speed(vec![312884470], 968709470), 1);

    // t153
    // https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/description/
    assert_eq!(Solution::find_min(vec![3 ,4, 5, 1, 2]), 1);
    assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);

    // t33
    // https://leetcode.com/problems/search-in-rotated-sorted-array/description/
    // shame on me
    assert_eq!(Solution::search_t33(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(Solution::search_t33(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    assert_eq!(Solution::search_t33(vec![1], 0), -1);
    assert_eq!(Solution::search_t33(vec![1], 1), 0);
    assert_eq!(Solution::search_t33(vec![3, 1], 0), -1);
    assert_eq!(Solution::search_t33(vec![3, 1], 1), 1);
    assert_eq!(Solution::search_t33(vec![3, 1], 4), -1);
    assert_eq!(Solution::search_t33(vec![4, 5, 6, 7, 0, 1, 2], 5), 1);
    assert_eq!(Solution::search_t33(vec![3, 5, 1], 6), -1);
    assert_eq!(Solution::search_t33(vec![3, 4, 5, 1, 2], 4), 1);

    // t981
    // https://leetcode.com/problems/time-based-key-value-store/description/
    let mut tm = TimeMap::new();
    tm.set("foo".to_string(), "bar".to_string(), 1);
    assert_eq!(tm.get("foo".to_string(), 1), "bar".to_string());
    assert_eq!(tm.get("foo".to_string(), 3), "bar".to_string());
    tm.set("foo".to_string(), "bar2".to_string(), 6);
    assert_eq!(tm.get("foo".to_string(), 6), "bar2".to_string());
    assert_eq!(tm.get("foo".to_string(), 7), "bar2".to_string());
    assert_eq!(tm.get("foo".to_string(), 5), "bar".to_string());

    // t4
    // https://leetcode.com/problems/median-of-two-sorted-arrays/description/
    assert_eq!(Solution::find_median_sorted_arrays(vec![2, 3], vec![2, 5, 5, 6, 6, 6]), 5.0);
    assert_eq!(Solution::find_median_sorted_arrays(vec![2, 4, 6], vec![1, 3, 5]), 3.5);
    assert_eq!(Solution::find_median_sorted_arrays(vec![2, 3, 6], vec![1, 3, 5]), 3.0);
    assert_eq!(Solution::find_median_sorted_arrays(vec![2], vec![1, 3, 5]), 2.5);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4], vec![1, 2, 3, 4]), 2.5);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4], vec![1, 2, 3, 3, 4]), 3.0);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4], vec![3, 4]), 3.0);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    assert_eq!(Solution::find_median_sorted_arrays(vec![], vec![1]), 1.0);
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]), 9.0);
}
