impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let r = query_row as usize;
        let c = query_glass as usize;
        let mut dp = vec![vec![0.0; r + 2]; r + 2];
        dp[0][0] = poured as f64;
        for i in 0..r {
            for j in 0..=i {
                let overflow = (dp[i][j] - 1.0).max(0.0);
                if overflow > 0.0 {
                    let cascade = overflow / 2.0;
                    dp[i + 1][j] += cascade;
                    dp[i + 1][j + 1] += cascade;
                }
            }
        }
        dp[r][c].min(1.0)
    }
}
