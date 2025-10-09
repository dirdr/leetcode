impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let mut dp = vec![0; n as usize + 1];
        dp[1] = 1;
        for i in 2..dp.len() {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n as usize]
    }
}
