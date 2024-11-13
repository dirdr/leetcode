impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        pub fn get_pair_count_less_than(nums: &Vec<i32>, target: i32) -> usize {
            let mut count = 0;
            let (mut left, mut right) = (0, nums.len() - 1);
            while left < right {
                if nums[left] + nums[right] < target {
                    count += right - left;
                    left += 1;
                } else {
                    right -= 1;
                }
            }
            count
        }
        let mut nums = nums;
        nums.sort_unstable();
        get_pair_count_less_than(&nums, upper + 1) as i64 - get_pair_count_less_than(&nums, lower) as i64
    }
}
