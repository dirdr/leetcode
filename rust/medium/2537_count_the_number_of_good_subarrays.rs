use std::collections::HashMap;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let mut answer = 0;
        let mut good_pairs = 0;
        let mut freq: HashMap<i32, i32> = HashMap::new();
        let mut left = 0;

        for right in 0..nums.len() {
            good_pairs += freq.get(&nums[right]).unwrap_or(&0);
            *freq.entry(nums[right]).or_insert(0) += 1;

            while good_pairs >= k {
                answer += (nums.len() - right) as i64;
                let removed = nums[left];
                let removed_count = freq.get(&removed).unwrap();
                good_pairs -= removed_count - 1;
                *freq.get_mut(&removed).unwrap() -= 1;
                left += 1;
            }
        }

        answer
    }
}
