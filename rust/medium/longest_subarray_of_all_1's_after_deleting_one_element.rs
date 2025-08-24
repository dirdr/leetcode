impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let (mut left, mut zeros, mut max_len) = (0, 0, 0);
        for (right, &num) in nums.iter().enumerate() {
            if num == 0 {
                zeros += 1;
            }
            while zeros > 1 {
                if nums[left] == 0 {
                    zeros -= 1;
                }
                left += 1;
            }
            max_len = max_len.max(right - left);
        }

        max_len as i32
    }
}
