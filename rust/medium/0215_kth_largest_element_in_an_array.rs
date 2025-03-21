use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut min_heap = BinaryHeap::new();
        for &el in &nums {
            min_heap.push(Reverse(el));
            if min_heap.len() > k {
                min_heap.pop();
            }
        }
        min_heap.peek().unwrap().0
    }
}
