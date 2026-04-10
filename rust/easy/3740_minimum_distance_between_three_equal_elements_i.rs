impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let mut min = usize::MAX;
        if nums.len() < 3 {
            return -1;
        }
        for i in 0..nums.len() - 2 {
            for j in i + 1..nums.len() - 1 {
                for k in j + 1..nums.len() {
                    if nums[i] == nums[j] && nums[i] == nums[k] && nums[j] == nums[k] {
                        min = min.min((i as isize - j as isize).abs() as usize + (j as isize - k as isize).abs() as usize + (k as isize - i as isize).abs() as usize);
                    }
                }
            }
        }
        min as i32
    }
}
