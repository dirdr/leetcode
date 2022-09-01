impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let nums_sum: i32 = nums.iter().sum::<i32>();
        let mut left_sum = 0;
        
        for i in 0..nums.len() {
            if nums_sum - left_sum - nums[i] == left_sum {
                return i as i32;
            }
            left_sum += nums[i];
        }
        -1
    }
}
