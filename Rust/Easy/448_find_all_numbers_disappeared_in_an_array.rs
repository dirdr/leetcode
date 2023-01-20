impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        for i in (0..nums.len()) {
            let index = (nums[i].abs() - 1) as usize;
            if nums[index] > 0 {
                nums[index] *= -1;
            }          
        }
        nums.into_iter()
            .enumerate()
            .filter(|(_, el)| *el > 0)
            .map(|(i, el)| (i + 1) as i32)
            .collect::<Vec<i32>>()
    }
}
