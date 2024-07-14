impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut answer = vec![];
        let mut used_idx = vec![false; nums.len()];
        fn backtrack(nums: &[i32], curr: &mut Vec<i32>, used_idx: &mut Vec<bool>, answer: &mut Vec<Vec<i32>>) {
            if curr.len() == nums.len() {
                answer.push(curr.clone());
            }
            for i in 0..nums.len() {
                if used_idx[i] {
                    continue;
                }
                if i > 0 && nums[i] == nums[i - 1] && !used_idx[i - 1] {
                    continue;
                }
                curr.push(nums[i]);
                used_idx[i] = true;
                backtrack(nums, curr, used_idx, answer);
                curr.pop();
                used_idx[i] = false;
                
            }
        }
        let mut nums = nums;
        nums.sort_unstable();
        backtrack(&nums, &mut vec![], &mut used_idx, &mut answer);
        answer
    }
}
