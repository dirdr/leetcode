use std::collections::HashSet;

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut indices = (0..nums.len()).collect::<Vec<_>>();
        indices.sort_unstable_by(|&a, &b| nums[b].cmp(&nums[a]));
        let set: HashSet<usize> = HashSet::from_iter(indices.into_iter().take(k as usize));
        (0..nums.len())
            .filter(|i| set.contains(&i))
            .map(|i| nums[i]).collect()
    }
}
