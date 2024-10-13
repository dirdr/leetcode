use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    i32,
};

#[derive(Eq, PartialEq)]
struct Node {
    val: i32,
    from: usize,
    idx: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let k = nums.len();
        let merged_size = nums.iter().map(|v| v.len()).sum::<usize>();
        let mut min_heap: BinaryHeap<Node> = BinaryHeap::new();
        let mut merged = Vec::with_capacity(merged_size);
        for i in 0..k {
            min_heap.push(Node {
                val: nums[i][0],
                from: i,
                idx: 0,
            });
        }
        while let Some(Node { val, from, idx }) = min_heap.pop() {
            merged.push((val, from));
            let next_idx = idx + 1;
            if next_idx < nums[from].len() {
                min_heap.push(Node {
                    val: nums[from][next_idx],
                    from,
                    idx: next_idx,
                });
            }
        }
        let (mut l, mut r) = (0, 0);
        let mut min_size = i32::MAX;
        let mut range = (i32::MAX, i32::MAX);
        let mut count = vec![0; k];
        let mut covered = 0;
        while r < merged.len() {
            let (right_val, right_from) = merged[r];
            if count[right_from] == 0 {
                covered += 1;
            }
            count[right_from] += 1;
            r += 1;
            while covered == k {
                let (left_val, left_from) = merged[l];
                let size = merged[r - 1].0 - left_val;
                if size < min_size || (size == min_size && left_val < range.0) {
                    min_size = size;
                    range = (left_val, merged[r - 1].0);
                }
                count[left_from] -= 1;
                if count[left_from] == 0 {
                    covered -= 1;
                }
                l += 1;
            }
        }
        vec![range.0, range.1]
    }
}
