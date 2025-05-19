impl Solution {
    pub fn triangle_type(mut nums: Vec<i32>) -> String {
        nums.sort_unstable();
        if nums[0] + nums[1] <= nums[2] {
            return "none".to_string();
        }
        match nums.chunk_by(|a, b| a == b).count() {
            1 => "equilateral".to_string(),
            2 => "isosceles".to_string(),
            _ => "scalene".to_string(),
        }
    }
}
