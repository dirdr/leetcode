impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        fn backtrack(nums: &[i32], current_xor: i32, pos: usize) -> i32 {
            let mut sum = current_xor;
            for i in pos..nums.len() {
                let new_xor = current_xor ^ nums[i];
                sum += backtrack(nums, new_xor, i + 1);
            }
            sum
        }
        backtrack(&nums, 0, 0)
    }
}
