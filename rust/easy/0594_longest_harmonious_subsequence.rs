use std::collections::HashMap;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for &num in &nums {
            *map.entry(num).or_insert(0) += 1;
        }
        let mut max = 0;
        for &num in &nums {
            if let Some(next) = map.get(&(num + 1)) {
                max = max.max(map.get(&num).unwrap() + next);
            }
        }
        max                                     
    }
}
