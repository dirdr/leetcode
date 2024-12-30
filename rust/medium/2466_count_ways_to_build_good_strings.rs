use std::collections::HashMap;

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        fn dfs(
            low: i32,
            high: i32,
            zero: i32,
            one: i32,
            cache: &mut HashMap<i32, i32>,
            value: i32,
        ) -> i32 {
            if let Some(&k) = cache.get(&value) {
                return k;
            }
            if value > high {
                return 0;
            }

            let left = dfs(low, high, zero, one, cache, value + zero);
            let right = dfs(low, high, zero, one, cache, value + one);

            let t = if value >= low && value <= high { 1 } else { 0 };

            let total = (left + right + t) % 1000000007;
            cache.insert(value, total);
            total
        }
        dfs(low, high, zero, one, &mut HashMap::new(), 0)
    }
}
