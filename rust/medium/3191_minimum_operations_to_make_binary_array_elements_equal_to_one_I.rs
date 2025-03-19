impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut operations = 0;
        let mut nums = nums;
        for i in 0..nums.len() - 2 {
            if nums[i] == 0 {
                nums[i] = 1;
                nums[i + 1] ^= 1;
                nums[i + 2] ^= 1;
                operations += 1;
            }
        }
        if nums.iter().sum::<i32>() == nums.len() as i32 { operations } else { -1 }
    }
}
