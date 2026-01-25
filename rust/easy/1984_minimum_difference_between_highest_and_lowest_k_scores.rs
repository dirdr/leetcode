impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut left = 0;
        let mut min = i32::MAX;
        for right in 0..nums.len() {
            if right - left + 1 >= k as usize {
                min = min.min(nums[right] - nums[left]);
                left += 1;
            }
        }
        min
    }
}
