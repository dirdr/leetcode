impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n < 3 {
            return 1;
        }
        let mut dp = vec![0; n as usize + 1];
        dp[1] = 1;
        dp[2] = 1;
        for i in 3..dp.len() {
            dp[i] = dp[i - 3] + dp[i - 2] + dp[i - 1];
        }
        dp[n as usize]
    }
}
