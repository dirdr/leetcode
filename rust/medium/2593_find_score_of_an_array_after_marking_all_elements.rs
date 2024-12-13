impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        let n = nums.len();
        let mut indices = (0..n).collect::<Vec<_>>();
        indices.sort_by(|&a, &b| nums[a].cmp(&nums[b]).then(a.cmp(&b)));
        let mut score = 0;
        for &i in &indices {
            if nums[i] > 0 {
                score += nums[i] as i64;
                nums[i] *= -1;
                if i > 0 && nums[i - 1] > 0 {
                    nums[i - 1] *= -1;
                }
                if i < nums.len() - 1 && nums[i + 1] > 0 {
                    nums[i + 1] *= -1;
                }
            }   
        }
        score
    }
}
