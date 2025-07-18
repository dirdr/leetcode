use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let n = nums.len() / 3;
        let mut min_prefix = vec![0i64; nums.len()];
        let mut max_suffix = vec![0i64; nums.len()];

        let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut current_sum = 0i64;

        for i in 0..nums.len() {
            if max_heap.len() < n {
                max_heap.push(nums[i]);
                current_sum += nums[i] as i64;
            } else if nums[i] < *max_heap.peek().unwrap() {
                current_sum -= max_heap.pop().unwrap() as i64;
                max_heap.push(nums[i]);
                current_sum += nums[i] as i64;
            }

            if max_heap.len() == n {
                min_prefix[i] = current_sum;
            }
        }

        let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        current_sum = 0;

        for i in (0..nums.len()).rev() {
            if min_heap.len() < n {
                min_heap.push(Reverse(nums[i]));
                current_sum += nums[i] as i64;
            } else if nums[i] > min_heap.peek().unwrap().0 {
                current_sum -= min_heap.pop().unwrap().0 as i64;
                min_heap.push(Reverse(nums[i]));
                current_sum += nums[i] as i64;
            }

            if min_heap.len() == n {
                max_suffix[i] = current_sum;
            }
        }

        let mut result = i64::MAX;
        for i in (n - 1)..(2 * n) {
            result = result.min(min_prefix[i] - max_suffix[i + 1]);
        }
        result
    }
}
