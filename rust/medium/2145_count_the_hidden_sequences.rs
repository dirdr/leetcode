impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut current: i64 = 0;
        let (mut min, mut max) = (0, 0);
        
        for &diff in &differences {
            current += diff as i64;
            min = min.min(current);
            max = max.max(current);
        }
        
        let min_start = lower as i64 - min;
        let max_start = upper as i64 - max;
        
        let result = std::cmp::max(0, max_start - min_start + 1);
        
        if result > i32::MAX as i64 {
            return 0;
        }
        
        result as i32
    }
}
