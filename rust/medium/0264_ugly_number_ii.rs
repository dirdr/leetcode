impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n];
        dp[0] = 1;
        let primes = [2, 3, 5];
        let mut indices = [0; 3];
        let mut next_values = [2, 3, 5];
        for i in 1..n {
            let min_value = *next_values.iter().min().unwrap();
            dp[i] = min_value;
            for (j, prime) in primes.iter().enumerate() {
                if next_values[j] == min_value {
                    indices[j] += 1;
                    next_values[j] = dp[indices[j]] * prime;
                }
            }
        }
        dp[n - 1]
    }
}
