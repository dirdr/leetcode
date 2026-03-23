impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![vec![i64::MAX, i64::MIN]; n]; m];
        dp[0][0] = vec![grid[0][0] as i64, grid[0][0] as i64];
        for i in 0..m {
            for j in 0..n {
                let curr = grid[i][j] as i64;
                if i > 0 {
                    dp[i][j][0] = dp[i][j][0].min(dp[i - 1][j][0] * curr);
                    dp[i][j][0] = dp[i][j][0].min(dp[i - 1][j][1] * curr);
                    dp[i][j][1] = dp[i][j][1].max(dp[i - 1][j][0] * curr);
                    dp[i][j][1] = dp[i][j][1].max(dp[i - 1][j][1] * curr);
                }
                if j > 0 {
                    dp[i][j][0] = dp[i][j][0].min(dp[i][j - 1][0] * curr);
                    dp[i][j][0] = dp[i][j][0].min(dp[i][j - 1][1] * curr);
                    dp[i][j][1] = dp[i][j][1].max(dp[i][j - 1][0] * curr);
                    dp[i][j][1] = dp[i][j][1].max(dp[i][j - 1][1] * curr);
                }
            }
        }
        let ans = dp[m - 1][n - 1][1];
        if ans < 0 { - 1 } else { (ans % 1_000_000_007) as i32 }
    }
}
