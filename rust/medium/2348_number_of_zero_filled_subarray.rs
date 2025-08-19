impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let (mut win, mut answer) = (0, 0);
        for r in 0..nums.len() {
            if r > 0 && nums[r] != 0 && nums[r - 1] == 0 {
                answer += (win * (win + 1)) / 2;
                win = 0;
            }
            if nums[r] == 0 {
                win += 1;
            }
        }
        if nums[nums.len() - 1] == 0 {
            answer += (win * (win + 1)) / 2;
        }
        answer
    }
}
