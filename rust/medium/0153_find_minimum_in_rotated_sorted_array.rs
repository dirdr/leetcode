impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let mid = l + (r - l) / 2;
            if nums[mid] > nums[r] {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        nums[l].min(nums[0])
    }
}
