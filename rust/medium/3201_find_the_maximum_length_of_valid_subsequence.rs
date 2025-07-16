impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut even = 0;
        let mut odd = 0;
        for i in 0..nums.len() {
            if nums[i] % 2 == 0 {
               even += 1;
            } else {
                odd += 1;
            }
        }
        let (mut alt_even, mut state) = (0, 0);
        for i in 0..nums.len() {
            if state == 0 && nums[i] % 2 == 0 {
                alt_even += 1;
                state = 1 - state;
            } else if state == 1 && nums[i] % 2 != 0 {
                alt_even += 1;
                state = 1 - state;
            }
        }
        let (mut alt_odd, mut state) = (0, 1);
        for i in 0..nums.len() {
            if state == 0 && nums[i] % 2 == 0 {
                alt_odd += 1;
                state = 1 - state;
            } else if state == 1 && nums[i] % 2 != 0 {
                alt_odd += 1;
                state = 1 - state;
            }
        }
        even.max(odd).max(alt_even).max(alt_odd)
    }
}
