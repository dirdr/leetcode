use std::collections::HashSet;

impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());
        let mut max = 0;
        for &num in &nums {
            let mut curr = num;
            let mut len = 1;
            while let Some(result) = curr.checked_pow(2) {
                if !set.contains(&result) {
                    break;
                }
                curr = result;
                len += 1;
            }
            max = max.max(len);
        }
        if max < 2 {
            return -1;
        }
        max
    }
}
