impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let mut answer = 0;
        let mut total = nums.iter().sum::<i32>();
        let mut current = 0;
        for i in 0..nums.len() - 1{
            current += nums[i];
            total -= nums[i];
            if (total - current) % 2 == 0 {
                answer += 1;
            }
        }
        answer
    }
}
