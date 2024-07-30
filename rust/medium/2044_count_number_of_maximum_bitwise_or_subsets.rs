impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        fn backtrack(nums: &[i32], max_or: i32, curr_or: i32, pos: usize) -> i32 {
            if curr_or == max_or {
                return 1 << (nums.len() - pos);
            }
            let mut count = 0;
            for i in pos..nums.len() {
                let or = curr_or | nums[i];
                count += backtrack(nums, max_or, or, i + 1);
            }
            count
        }
        let max_or = nums.iter().fold(0, |acc, &x| acc | x);
        backtrack(&nums, max_or, 0, 0)
    }
}
