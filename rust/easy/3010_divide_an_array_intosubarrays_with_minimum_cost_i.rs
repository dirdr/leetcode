impl Solution {
    pub fn minimum_cost(mut nums: Vec<i32>) -> i32 {
        let mut sum = nums[0];
        let mut min = i32::MAX;
        for j in 1..nums.len() - 1 {
            for k in j + 1..nums.len() {
                min = min.min(sum + nums[j] + nums[k]);
            }
        }
        min
    }
}
