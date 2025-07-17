impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![vec![0; k]; k];
        let mut max = 0;
        for &num in &nums {
            let cr = num as usize % k;
            for target in 0..k {
                let cor = (target - cr + k) % k;
                dp[cr][cor] = dp[cor][cr] + 1;
                max = max.max(dp[cr][cor]);
            }
        }
        max
    }
}
