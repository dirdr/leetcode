impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, 0);
        let (mut ans, mut bitmask) = (0, 0);
        while right < nums.len() {
            while (bitmask & nums[right]) != 0 {
                bitmask ^= nums[left];
                left += 1;
            }
            bitmask |= nums[right];
            ans = ans.max(right - left + 1);
            right += 1
        }
        ans as i32
    }
}
