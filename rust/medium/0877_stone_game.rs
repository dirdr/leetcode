impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let n = piles.len();
        let mut dp = vec![vec![0; n]; n];
        for i in 0..n {
            dp[i][i] = piles[i];
        }
        for i in (0..n).rev() {
            for j in i + 1..n {
                let left = if i + 1 <= j {
                    piles[i] + piles[(i + 1)..=j].iter().sum::<i32>() - dp[i + 1][j]
                } else {
                    0
                };
                let right = if i <= j - 1 {
                    piles[j] + piles[i..j].iter().sum::<i32>() - dp[i][j - 1]
                } else {
                    0
                };
                dp[i][j] = std::cmp::max(left, right);
            }
        }
        let total_stones: i32 = piles.iter().sum();
        dp[0][n - 1] > total_stones / 2
    }
}
