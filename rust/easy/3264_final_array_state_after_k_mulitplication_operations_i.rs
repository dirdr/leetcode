use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
struct Node {
    index: usize,
    value: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.value.cmp(&self.value).then_with(|| other.index.cmp(&self.index))
    }
 }

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let mut nums = nums;
        let mut heap = BinaryHeap::from_iter(nums.iter().enumerate().map(|(index, &value)| Node {
            index, value,
        }));
        for _ in 0..k {
            let min = heap.pop().unwrap();
            let multiplied = min.value * multiplier;
            nums[min.index] = multiplied;
            heap.push(Node {
                index: min.index,
                value: multiplied,
            });
        }
        nums
    }
}
