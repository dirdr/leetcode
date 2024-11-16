impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if nums.len() == 1 || k == 1 {
            return nums;
        }
        let k = k as usize;
        let mut answer = vec![-1; nums.len() - k + 1];
        let mut len = 1;
        for i in 1..nums.len() {
            len = if nums[i] - nums[i - 1] == 1 { len + 1 } else { 1 };
            if len >= k {
                answer[i - k + 1] = nums[i];
            }
        }
        answer
    }
}
