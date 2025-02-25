impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let (mut sum, mut odd, mut even, mut result) = (0, 0, 0, 0);
        let MOD = 1_000_000_007;
        for &el in &arr {
            sum += el;
            if sum % 2 == 0 {
                result = (result + odd) % MOD;
                even += 1;
            } else {
                result = (result + 1 + even) % MOD;
                odd += 1;
            }
        }
        result
    }
}
