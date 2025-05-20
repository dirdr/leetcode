impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let n = nums.len();
        let mut diff = vec![0; n];
        for q in &queries {
            diff[q[0] as usize] -= 1;
            if q[1] + 1 < n as i32 {
                diff[q[1] as usize + 1] += 1;
            } 
        }
        let mut current = 0;
        for i in 0..n {
            current += diff[i];
            if nums[i] + current > 0 {
                return false;
            }
        }
        true
    }
}
