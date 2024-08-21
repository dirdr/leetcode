impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let n = s.len();
        if n == 0 {
            return 0;
        }
        let s = s.as_bytes();
        let mut dp = vec![vec![0; n]; n];
        for i in 0..n {
            dp[i][i] = 1;
        }
        for length in 2..=n {
            for i in 0..=n - length {
                let j = i + length - 1;
                dp[i][j] = dp[i][j - 1] + 1;
                for k in i..j {
                    if s[k] == s[j] {
                        dp[i][j] = dp[i][j].min(dp[i][k] + dp[k + 1][j - 1]);
                    }
                }
            }
        }
        dp[0][n - 1]
    }
}
