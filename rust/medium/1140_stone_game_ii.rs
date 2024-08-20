impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        let mut dp = vec![vec![0; n + 1]; n + 1];
        let mut suffix_sum = vec![0; n + 1];
        for i in (0..n).rev() {
            suffix_sum[i] = suffix_sum[i + 1] + piles[i];
        }
        for i in (0..n).rev() {
            for m in 1..=n {
                for x in 1..=(2 * m) {
                    if i + x > n {
                        break;
                    }
                    let bob_play = if i + x < n {
                        dp[i + x][m.max(x) - 1]
                    } else {
                        0
                    };
                    dp[i][m - 1] = dp[i][m - 1].max(suffix_sum[i] - bob_play);
                }
            }
        }
        dp[0][0]
    }
}
