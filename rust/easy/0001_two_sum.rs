use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, el) in nums.iter().enumerate() {
            if let Some(idx) = map.get(el) {
                return vec![*idx, i as i32];
            }
            map.insert(target - *el, i as i32);
        }
        unreachable!()
    }
}
