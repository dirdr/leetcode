impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
        let mut valid = 0;
        for i in 0..nums.len() {
            let mut count = 0;
            for j in i..nums.len() {
                if nums[j] == target {
                    count += 1;
                }
                if count * 2 > j - i + 1 {
                    valid += 1;
                }
            }
        }
        valid
    }
}
