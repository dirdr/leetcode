impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut count = 0;
        for right in 0..nums.len() {
            if right - left + 1 < 3 {
                continue
            }
            let middle = nums[right - 1];
            if middle % 2 == 0 && nums[right] + nums[left] == middle / 2 {
                count += 1
            }
            left += 1;

        }
        count
    }
}
