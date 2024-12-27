impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let n = values.len();
        if n == 2 {
            return values[0] + values[1] - 1;
        }
        let mut dp = values[0];
        let mut max = 0;
        for i in 1..n {
           max = max.max(dp + values[i] - i as i32);
           dp = dp.max(values[i] + i as i32); 
        }
        max
    }
}
