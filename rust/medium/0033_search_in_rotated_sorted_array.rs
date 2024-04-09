impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let (mut l, mut r) = (0 as usize, nums.len() - 1);
        while l <= r {
            let m = l + (r - l) / 2;
            if nums[m] == target {
                return m as i32;
            }
            // left half that is sorted
            if nums[l] <= nums[m] {
                // target is in the left half
                if target >= nums[l] && target <= nums[m] {
                    r = m - 1;
                // target is in the half with pivot
                } else {
                    l = m + 1;
                }
            // right half is sorted
            } else {
                // if the target is in the right half
                if target >= nums[m] && target <= nums[r] {
                    l = m + 1;
                // target is in the half with pivot
                } else {
                    r = m - 1;
                }
            }
        }
        -1
    }
}
