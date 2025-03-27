use std::collections::HashMap;

impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut first_map = HashMap::<i32, i32>::new();
        let mut second_map = HashMap::<i32, i32>::new();
        for &el in &nums {
            second_map.entry(el).and_modify(|e| *e += 1).or_insert(1);
        }
        for idx in 0..n {
            let num = nums[idx as usize];
            second_map.entry(num).and_modify(|e| *e -= 1);
            first_map.entry(num).and_modify(|e| *e += 1).or_insert(1);
            if first_map[&num] * 2 > idx + 1 && second_map[&num] * 2 > n - idx - 1 {
                return idx;
            }
        }
        -1
    }
}
