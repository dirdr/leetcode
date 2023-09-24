impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len: usize = nums.len();
        let mut prefix = vec![1; len];       
        let mut suffix = vec![1; len];
        let mut answer = vec![1; len];
        for i in 1..len {
            prefix[i] = prefix[i - 1] * nums[i - 1];
            suffix[len - i - 1] = suffix[len - i] * nums[len - i];
        }
        for i in 0..len {
            answer[i] = prefix[i] * suffix[i];
        }
        answer
    }
}
