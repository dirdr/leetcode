impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let (mut left, mut freq, mut answer) = (0, 0, 0);
        let max = nums.iter().max().unwrap();
        for right in 0..nums.len() {
            let entry = nums[right];
            if &entry == max {
                freq += 1;
            }
            while freq >= k {
                answer += (nums.len() - right) as i64;
                if &nums[left] == max {
                    freq -= 1;
                }
                left += 1;
            }
        }
        answer
    }
}
