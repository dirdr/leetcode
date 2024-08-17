impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        if obstacle_grid[0][0] == 1 || obstacle_grid[m - 1][n - 1] == 1 {
            return 0;
        }
        let mut dp = vec![0; n];
        dp[0] = 1;
        for r in 0..m {
            for c in 0..n {
                if obstacle_grid[r][c] == 1 {
                    dp[c] = 0;
                } else if c > 0 {
                    dp[c] += dp[c - 1];
                }
            }
        }
        dp[n - 1]
    }
}
