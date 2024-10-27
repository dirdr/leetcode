impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; n]; m];
        dp[0][0] = matrix[0][0];
        let mut count = matrix[0][0];
        for i in 1..m {
            dp[i][0] = matrix[i][0];
            count += dp[i][0];
        }
        for j in 1..n {
            dp[0][j] = matrix[0][j];
            count += dp[0][j];
        }
        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] != 1 {
                    continue;
                }
                dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1;
                count += dp[i][j];
            }
        }
        count
    }
}
