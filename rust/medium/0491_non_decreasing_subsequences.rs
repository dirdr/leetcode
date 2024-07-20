impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(nums: &[i32], answer: &mut Vec<Vec<i32>>, current: &mut Vec<i32>, idx: usize) {
            if current.len() >= 2 && !answer.contains(&current) {
                answer.push(current.clone());
            }
            for i in idx..nums.len() {
                if current.len() == 0 || nums[i] >= *current.last().unwrap() {
                    current.push(nums[i]);
                    backtrack(nums, answer, current, i + 1);
                    current.pop();
                }
            }            
        }
        let mut answer = vec![];
        backtrack(&nums, &mut answer, &mut vec![] , 0);
        answer
    }
}
