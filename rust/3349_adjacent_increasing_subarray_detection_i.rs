impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let (mut current_len, mut previous_len) = (1, 0);
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                current_len += 1;
            } else {
                previous_len = current_len;
                current_len = 1;
            }
            if (current_len >= k && previous_len >= k) || current_len >= 2 * k {
                return true;
            }
        }
        false
    }
}
