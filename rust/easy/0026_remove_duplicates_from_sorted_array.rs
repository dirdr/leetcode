use std::collections::HashSet;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut unique_idx = 0;
        let mut seen: HashSet<i32> = HashSet::new();
        for i in 0..nums.len() {
            if !seen.contains(&nums[i]) {
                seen.insert(nums[i]);
                let temp = nums[unique_idx];
                nums[unique_idx] = nums[i];
                nums[i] = temp;
                unique_idx += 1;
            }
        }
        unique_idx as i32
    }
}
