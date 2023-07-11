use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m = HashMap::new();
        for (i, el) in nums.iter().enumerate() {
            if m.contains_key(el) {
                return vec![*m.get(el).unwrap(), i as i32];
            }
            m.insert(target - el, i as i32);
        }
        Vec::new()
    }
}
