impl Solution {
    pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }
        }
        let mut ans = vec![0; nums.len()];
        let mut curr = 0;
        for &el in &nums {
            if el != 0 {
                ans[curr] = el;
                curr += 1;
            }
        }
        ans
    }
}
