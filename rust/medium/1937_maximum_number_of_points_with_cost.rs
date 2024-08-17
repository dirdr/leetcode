impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let (m, n) = (points.len(), points[0].len());
        let mut dp = vec![vec![0i64; n]; m];
        for c in 0..n {
            dp[m - 1][c] = points[m - 1][c] as i64;
        }
        for r in (0..m - 1).rev() {
            let mut going_left = vec![0; n];
            let mut going_right = vec![0; n];
            going_left[0] = dp[r + 1][0];
            for c in 1..n {
                going_left[c] = dp[r + 1][c].max(going_left[c - 1] - 1);
            }
            going_right[n - 1] = dp[r + 1][n - 1];
            for c in (0..n - 1).rev() {
                going_right[c] = dp[r + 1][c].max(going_right[c + 1] - 1);
            }
            for c in 0..n {
                dp[r][c] = points[r][c] as i64 + going_left[c].max(going_right[c]);
            }
        }
        *dp[0].iter().max().unwrap() as i64
    }
}
