impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let (mut min, mut max) = (arrays[0][0], arrays[0][arrays[0].len() - 1]);
        let mut diff = 0; 
        for nums in arrays.iter().skip(1) {
            let (left, right) = (nums[0], nums[nums.len() - 1]);
            diff = diff.max(right - min).max(max - left);
            min = min.min(left);
            max = max.max(right);
        }
        diff
    }
}
