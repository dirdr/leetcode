impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1; nums.len()];
        for i in (0..nums.len()).rev() {
            for j in i + 1..nums.len() {
                if nums[j] > nums[i] {
                    dp[i] = std::cmp::max(dp[i], 1 + dp[j]);
                }
            }
        }
        *dp.iter().max().unwrap()
    }
}
