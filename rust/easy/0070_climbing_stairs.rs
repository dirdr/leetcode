impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // s(i) is the number of way for the stair i
        // When on i stair, i could have gone from i - 1 stair or from i - 2 stair.
        // So there is s(i - 1) + s(i - 2) possible way 
        // you need to think recursively, while we solve this bottom up way
        let mut dp = vec![1; n as usize + 1];
        for i in 2..dp.len() {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n as usize]
    }
}
