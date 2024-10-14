use std::collections::BinaryHeap;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut max_heap = BinaryHeap::new();
        for el in nums {
            max_heap.push(el);
        }
        let mut score = 0i64;
        for i in 0..k {
            let max = max_heap.pop().unwrap();
            score += max as i64;
            max_heap.push((max as f64 / 3.0).ceil() as i32);
        }
        score
    }
}
