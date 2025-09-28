impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        for i in (2..nums.len()).rev() {
            let (a, b, c) = (nums[i - 2], nums[i - 1], nums[i]);
            if a + b > c {
                return a + b + c
            }
        }
        0
    }
}
