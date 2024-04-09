use std::cmp;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;
        let (mut l, mut r) = (0 as usize, height.len() - 1);
        while l < r {
            max = cmp::max(max, (r - l) * cmp::min(height[r], height[l]) as usize);
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        max as i32
    }
}
