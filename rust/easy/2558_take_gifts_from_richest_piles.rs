use std::collections::BinaryHeap;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut heap = BinaryHeap::from(gifts.iter().map(|&e| e as i64).collect::<Vec<_>>());
        for i in 0..k {
            if let Some(max) = heap.pop() {
                heap.push((max as f64).sqrt().floor() as i64);
            }
        }
        heap.into_iter().sum::<i64>()
    }
}
