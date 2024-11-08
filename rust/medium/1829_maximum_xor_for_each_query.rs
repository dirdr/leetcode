impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut max = 2_i32.pow(maximum_bit as u32) - 1;
        let mut nums = nums;
        let mut answer = Vec::with_capacity(nums.len());
        let mut curr_xor = 0;
        for &el in &nums {
            curr_xor ^= el;
            answer.push(curr_xor ^ max);
        }
        answer.reverse();
        answer
    }
}
