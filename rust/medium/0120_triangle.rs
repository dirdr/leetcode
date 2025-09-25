impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut dp = triangle[n - 1].clone();
        for row in (0..n - 1).rev() {
            for col in 0..row + 1 {
                dp[col] = triangle[row][col] + dp[col].min(dp[col + 1]);
            }
        }
        dp[0]
    }
}
