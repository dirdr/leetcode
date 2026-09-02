impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 3 {
            return nums[0] * nums[1] * nums[2];
        }
        nums.sort_unstable();
        let n = nums.len();
        let (min1, min2) = (nums[0], nums[1]);
        let (max1, max2, max3) = (nums[n - 1], nums[n - 2], nums[n - 3]);
        let pn = min1 * min2;
        (max1 * max2 * max3).max(min1 * min2 * max1)
    }
}
