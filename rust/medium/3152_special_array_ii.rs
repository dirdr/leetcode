impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = nums.len();
        let mut prefix = vec![0; nums.len()];
        for i in 1..nums.len() {
            let special = (nums[i - 1] + nums[i]) % 2 == 0;
            prefix[i] = prefix[i - 1] + special as i32;
        }
        queries.iter()
            .map(|q| prefix[q[1] as usize] - prefix[q[0] as usize] == 0)
            .collect::<Vec<_>>()
    }
}
