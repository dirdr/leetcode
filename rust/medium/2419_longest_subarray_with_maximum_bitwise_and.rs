impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let max = nums.iter().max().unwrap();
        nums.iter()
            .fold((0, 0), |(max_len, curr_len), &num| {
                if num == *max {
                    let new_len = curr_len + 1;
                    (max_len.max(new_len), new_len)
                } else {
                    (max_len, 0)
                }
            })
            .0
    }
}
