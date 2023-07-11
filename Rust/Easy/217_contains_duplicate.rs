use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        nums.len() != HashSet::<i32>::from_iter(nums).len()
    }
}
