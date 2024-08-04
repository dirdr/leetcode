impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut sums = Vec::new();
        for i in 0..nums.len() {
            let mut sum = 0;
            for &num in nums[i..].iter() {
                sum = (sum + num) % MOD;
                sums.push(sum);
            }
        }
        sums.sort_unstable();
        sums[(left - 1) as usize..(right) as usize]
            .iter()
            .fold(0, |acc, &x| (acc + x) % MOD)
    }
}
