impl Solution {
    pub fn max_product(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.reverse();
        (nums[0] - 1) * (nums[1] - 1)
    }
}
