use std::cmp::Ordering;

impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        match (z - x).abs().cmp(&(z - y).abs()) {
            Ordering::Equal => 0,
            Ordering::Less => 1,
            Ordering::Greater => 2,
        }
    }
}
