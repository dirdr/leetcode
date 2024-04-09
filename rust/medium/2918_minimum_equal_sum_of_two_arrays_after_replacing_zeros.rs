use std::cmp::Ordering;

impl Solution {
    
    pub fn min_sum_after_completion(nums: &[i32]) -> i64 {
        nums.iter().fold(0i64, |acc, &x| acc + (if x == 0 {1} else {x}) as i64)
    }
    
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let mut min_sum_nums1: i64 = Self::min_sum_after_completion(&nums1);
        let mut min_sum_nums2: i64 = Self::min_sum_after_completion(&nums2);
        let (min_array, max_array) = match min_sum_nums1.cmp(&min_sum_nums2) {
            Ordering::Less => (nums1, nums2),
            Ordering::Greater => (nums2, nums1),
            Ordering::Equal => return min_sum_nums1,
        };
        let avaible_min_array = min_array.iter().filter(|&&x| x == 0).count();
        let a = Self::min_sum_after_completion(&max_array);
        if avaible_min_array == 0 {
            return -1;
        }
        a
    }
}
