impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut answer = vec![-1; n];
        for i in 0..n {
            answer[i] = nums[nums[i] as usize];
        }
        answer
    }
}
