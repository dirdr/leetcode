impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        fn dfs(nums: &[i32], k: i32, curr: &mut Vec<i32>, pos: usize) -> i32 {
            let n = curr.len();
            if n >= 2 {
                let last = curr[n - 1];
                for &num in &curr[..n - 1] {
                    if (num - last).abs() == k {
                        return 0;
                    }
                }
            }
            let mut count = 1;
            for i in pos..nums.len() {
                curr.push(nums[i]);
                count += dfs(nums, k, curr, i + 1);
                curr.pop();
            }
            count
        }
        dfs(&nums, k, &mut vec![], 0) - 1
    }
}
