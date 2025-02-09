use std::collections::HashMap;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let mut map = HashMap::new();
        for (i, el) in nums.iter().enumerate() {
            map.entry(el - i as i32)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        let mut good_pairs = map
            .values()
            .filter(|&&v| v > 1)
            .map(|&v| (v * (v - 1)) / 2)
            .sum::<i64>();
        let n = nums.len() as i64;
        (n * (n - 1)) / 2 - good_pairs
    }
}
