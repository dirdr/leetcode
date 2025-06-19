impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut answer = 0;
        let mut nums = nums;
        nums.sort_unstable();
        let mut current = 0;
        for i in 1..nums.len() {
            if nums[i] - nums[current] > k {
                current = i;
                answer += 1;
            }
        }
        answer + 1
    }
}
