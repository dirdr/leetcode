impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let mut min = i32::MAX;
        for i in 0..nums.len() {
            if nums[i] == target {
                min = min.min((i as isize - start as isize).abs() as i32);
            }
        }
        min
    }
}
