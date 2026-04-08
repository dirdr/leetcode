impl Solution {
    pub fn xor_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        for q in queries {
            let mut i = q[0] as usize;
            while i <= q[1] as usize {
                nums[i] = ((nums[i] as i64 * q[3] as i64) % MOD as i64) as i32;
                i += q[2] as usize;
            }
        }
        let mut xor = nums[0];
        for i in 1..nums.len() {
            xor ^= nums[i];
        }
        xor
    }
}
