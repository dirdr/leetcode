impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        let first = nums[0];
        let mut nums = nums;
        nums.push(first);
        let mut max = 0;
        for i in 1..nums.len() {
            max = max.max((nums[i] - nums[i - 1]).abs());
        }
        max
    }
}
