impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        nums.into_iter().map(|num| {
            sum += num;
            sum
        }).collect()
    }
}
