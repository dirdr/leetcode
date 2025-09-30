impl Solution {
    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        let mut dp = nums;
        let n = dp.len();
        for k in 1..n {
            for i in 0..n - k {
                dp[i] = (dp[i] + dp[i + 1]) % 10
            }
        }
        dp[0]
    }
}
