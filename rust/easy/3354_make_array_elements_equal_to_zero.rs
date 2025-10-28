impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let is_valid = |mut idx: isize, mut dir: isize| {
            let mut sandbox = nums.clone();
            while idx >= 0 && (idx as usize) < sandbox.len() {
                if sandbox[idx as usize] > 0 {
                    sandbox[idx as usize] -= 1;
                    dir *= -1;
                }
                idx += dir;
            }
            sandbox.iter().all(|&e| e == 0)
        };
        let mut answer = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                answer += match (is_valid(i as isize, 1), is_valid(i as isize, -1)) {
                    (true, true) => 2,
                    (true, false) => 1,
                    (false, true) => 1,
                    (false, false) => 0,
                };
            }
        }
        answer
    }
}
