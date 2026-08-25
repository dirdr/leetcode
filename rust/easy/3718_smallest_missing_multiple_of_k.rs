use std::collections::HashSet;

impl Solution {
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        let set: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());
        for i in 1..=i32::MAX {
            let i = i as i32;
            if !set.contains(&i) && i % k == 0 {
                return i;
            }
        }
        -1
    }
}
