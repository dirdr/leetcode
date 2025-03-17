impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        nums.sort();
        nums.chunks(2).all(|w| w[0] == w[1])
    }
}
