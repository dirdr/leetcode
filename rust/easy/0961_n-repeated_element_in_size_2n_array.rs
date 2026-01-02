use std::collections::HashSet;

impl Solution {
    pub fn repeated_n_times(mut nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();
        for &el in &nums {
            if !set.insert(el) {
                return el;
            }
        }
        unreachable!()
    }
}
