use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            match (map.get(&nums[i])) {
                Some(val) => {
                    let diff: i32 = i as i32 - *val;
                    if i32::abs(diff) <= k && i as i32 != *val { return true; }
                }
                None => {}
            }
            map.insert(nums[i], i as i32);
        }
        false
    }
}
