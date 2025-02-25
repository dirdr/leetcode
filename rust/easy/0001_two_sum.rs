use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, &el) in nums.iter().enumerate() {
            let needed = target - el;
            if let Some(&idx) = map.get(&needed) {
                return vec![i as i32, idx as i32];
            } else {
                map.insert(el, i);
            }
        }
        unreachable!()
    }
}
