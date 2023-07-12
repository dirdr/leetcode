use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count = nums.iter().fold(HashMap::new(), |mut map, e| {
            map.entry(e).and_modify(|e| *e += 1).or_insert(1);
            map
        });
        let mut sorted = count.clone().keys().map(|&&e| e).collect::<Vec<i32>>();
        sorted.sort_unstable_by(|a, b| count.get(b).unwrap().cmp(count.get(a).unwrap()));
        sorted.into_iter()
            .take(k as usize)
            .collect::<Vec<i32>>()
    }
}
