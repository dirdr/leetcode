impl Solution {
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let (mut left, mut right) = (0, *nums.last().unwrap() - nums[0]);
        while left < right {
            let mid = left + (right - left) / 2;
            if Self::count_pairs(&nums, mid) >= k {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
    
    fn count_pairs(nums: &[i32], guess: i32) -> i32 {
        let (mut count, mut left) = (0, 0);
        for right in 0..nums.len() {
            while nums[right] - nums[left] > guess {
                left += 1;
            }
            count += right - left;
        }
        count as i32
    }
}
