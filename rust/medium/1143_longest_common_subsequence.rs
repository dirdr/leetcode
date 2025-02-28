impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1 = text1.chars().collect::<Vec<_>>();
        let text2 = text2.chars().collect::<Vec<_>>();
        let (n, m) = (text1.len(), text2.len());
        let mut dp = vec![vec![0; m + 1]; n + 1];
        for i in (0..n).rev() {
            for j in (0..m).rev() {
                if text1[i] == text2[j] {
                    dp[i][j] = 1 + dp[i + 1][j + 1];
                } else {
                    dp[i][j] = std::cmp::max(dp[i][j + 1], dp[i + 1][j]);
                }
            }
        }
        dp[0][0]
    }
}
