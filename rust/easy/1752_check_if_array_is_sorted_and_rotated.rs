impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        nums.extend(nums.clone().iter());
        let cut = nums.windows(2).filter(|&w| w[0] > w[1]).count();
        cut <= 2
    }
}
