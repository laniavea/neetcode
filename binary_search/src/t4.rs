use crate::Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {

        let (nums1, nums2) = if nums1.len() > nums2.len() {
            (nums2, nums1)
        } else {
            (nums1, nums2)
        };

        let half_len = (nums1.len() + nums2.len() + 1) / 2;

        let mut left_br = 0;
        let mut right_br = nums1.len();

        while left_br <= right_br {
            let i = (left_br + right_br) / 2;
            let j = half_len - i;

            let nums1_left_max = if i == 0 { i32::MIN } else { nums1[i - 1] };
            let nums1_right_min = if i == nums1.len() { i32::MAX } else { nums1[i] };

            let nums2_left_max = if j == 0 { i32::MIN } else { nums2[j - 1] };
            let nums2_right_min = if j == nums2.len() { i32::MAX } else { nums2[j] };

            if nums1_left_max <= nums2_right_min && nums2_left_max <= nums1_right_min {
                if (nums1.len() + nums2.len()) % 2 == 0 {
                    return (nums1_left_max.max(nums2_left_max) + nums1_right_min.min(nums2_right_min)) as f64 / 2.0;
                } else {
                    return nums1_left_max.max(nums2_left_max) as f64;
                }
            } else if nums1_left_max > nums2_right_min {
                right_br = i - 1;
            } else {
                left_br = i + 1;
            }
        }
        unreachable!();
    }
}
