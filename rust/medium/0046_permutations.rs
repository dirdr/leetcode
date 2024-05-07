use std::collections::HashSet;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(nums: &Vec<i32>, current: &mut Vec<i32>, answer: &mut Vec<Vec<i32>>) {
            if current.len() == nums.len() {
                answer.push(current.clone());
                return;
            }
            if current.len() > nums.len() {
                return;
            }
            for i in 0..nums.len() {
                if !current.contains(&nums[i]) {
                    current.push(nums[i]);
                    backtrack(nums, current, answer);
                    current.pop();
                }
            }
        }
        let mut answer = vec![];
        backtrack(&nums, &mut vec![], &mut answer);
        answer
    }
}
