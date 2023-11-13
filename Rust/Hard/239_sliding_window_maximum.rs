use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k as usize >= nums.len() {
            return vec![*nums.iter().max().unwrap()];
        }
        let mut answer: Vec<i32> = vec![];
        let mut queue: VecDeque<usize> = VecDeque::new();
        let (mut l, mut r) = (0usize, 0usize);
        while r < nums.len() {
            while let Some(&i) = queue.back() {
                if nums[*queue.back().unwrap()] > nums[r] {
                    break;
                }
                queue.pop_back();
            }
            queue.push_back(r);
            if l > *queue.front().unwrap() {
                queue.pop_front();
            }
            if r >= (k - 1) as usize {
                answer.push(nums[*queue.front().unwrap()]);
                l += 1;
            }
            r += 1;
        }
        answer
    }
}
