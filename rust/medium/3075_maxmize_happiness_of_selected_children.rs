impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
        happiness.sort_unstable_by(|a, b| b.cmp(&a));
        for i in 1..happiness.len() {
            happiness[i] = 0.max(happiness[i] - i as i32);
        }
        happiness.into_iter().take(k as usize).map(|e| e as i64).sum::<i64>()
    }
}
