use std::collections::HashMap;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for n in nums.iter().copied() {
            map.entry(n).and_modify(|e| *e += 1).or_insert(1);
        }
        let me = map.iter().map(|(_, v)| v).max().unwrap();
        nums.iter().filter(|e| map.get(e).unwrap() == me).count() as i32
    }
}
