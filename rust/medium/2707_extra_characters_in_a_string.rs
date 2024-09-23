use std::collections::HashSet;

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let n = s.len();
        let mut dp = vec![0; n + 1];
        let dict_set: HashSet<&str> = dictionary.iter().map(|s| s.as_str()).collect();
        for r in (0..n).rev() {
            dp[r] = dp[r + 1] + 1;
            for len in 1..=n - r {
                if dict_set.contains(&s[r..r + len]) {
                    dp[r] = dp[r].min(dp[r + len]);
                }
            }
        }
        dp[0]
    }
}
