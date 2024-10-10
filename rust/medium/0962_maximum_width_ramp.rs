impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        // O(N) TC - O(N) SC
        let mut indices = (0..nums.len()).collect::<Vec<_>>();
        indices.sort_by(|&a, &b| nums[a].cmp(&nums[b]));
        let mut min_idx = indices[0];
        let mut max = 0;
        for i in 1..indices.len() {
            min_idx = min_idx.min(indices[i]);
            max = max.max(indices[i] - min_idx);
        }
        max as i32
    }
}a
