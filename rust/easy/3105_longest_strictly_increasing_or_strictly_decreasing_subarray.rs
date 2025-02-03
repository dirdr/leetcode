impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        }
        let mut streak = 1;
        let mut max = 0;
        for w in nums.windows(2) {
            if w[0] > w[1] {
                streak += 1;
            } else {
                streak = 1;
            }
            max = max.max(streak);
        }
        streak = 1;
        for w in nums.windows(2) {
            if w[0] < w[1] {
                streak += 1;
            } else {
                streak = 1;
            }
            max = max.max(streak);
        }
        max
    }
}
