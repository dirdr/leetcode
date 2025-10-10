impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let n = energy.len();
        let mut dp = vec![0; n];
        let mut max = i32::MIN;
        let k = k as usize;
        for i in 0..n {
            if i < k {
                dp[i] = energy[i];
            } else {
                dp[i] = energy[i].max(energy[i] + dp[i - k]);
            }
            if i >= n - k {
                max = max.max(dp[i]);
            }
        }
        max
    }
}
