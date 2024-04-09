impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0 as usize, nums.len() - 1);
        if nums[right] > nums[left] {
            return nums[left];
        }
        let mut min = nums[left];
        while left <= right {
            if nums[left] < nums[right] {
                min = std::cmp::min(nums[left], min);
            }
            let middle = left + (right - left) / 2;
            min = std::cmp::min(nums[middle], min);
            if nums[middle] >= nums[left] {
                left = middle + 1;
            } else {
                right = middle - 1;
            }
        }
        min
    }
}
