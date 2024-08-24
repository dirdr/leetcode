impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut dp = vec![vec![1]];
        if num_rows == 1 {
            return dp;
        }
        dp.push(vec![1, 1]);
        if num_rows == 2 {
            return dp;
        }
        for i in 2..num_rows as usize {
            dp.push(vec![1; i + 1]);
            for j in 1..i {
                dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
            }
        }
        dp
    }
}
