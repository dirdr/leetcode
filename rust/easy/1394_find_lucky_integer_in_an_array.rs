use std::collections::HashMap;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for &el in &arr {
            *map.entry(el).or_insert(0) += 1;
        }
        map.into_iter()
            .filter(|(k, v)| k == v)
            .map(|(k, _)| k)
            .max()
            .unwrap_or(-1)
    }
}
