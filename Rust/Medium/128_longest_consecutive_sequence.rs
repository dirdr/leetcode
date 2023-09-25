use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());
        let mut max = 0;
        for el in nums.iter() {
            let mut current = 1;
            let mut i = 1;
            if !set.contains(&(el - 1)) {
                while set.contains(&(el + i)) {
                    i += 1;
                    current += 1;
                }
            }
            max = if current >= max {current} else {max}
        }
        max
    }
}
