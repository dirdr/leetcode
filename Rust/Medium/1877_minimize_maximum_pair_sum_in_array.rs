impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums.clone();
        nums.sort();
        let mut left: usize = 0;
        let mut right: usize = nums.len() - 1;
        let mut max: usize = 0;
        while (left <= right) {
            let pair_sum = nums[left] as usize + nums[right] as usize;
            if pair_sum >= max {
                max = pair_sum;
            }
            left += 1;
            right -= 1;
        }
        max as i32
    }
}
