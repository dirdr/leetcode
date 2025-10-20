impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp = cost.clone();
        let n = dp.len();
        for i in 2..cost.len() {
            dp[i] = dp[i - 1].min(dp[i - 2]) + cost[i];
        }
        dp[n - 1].min(dp[n - 2])
    }
}
