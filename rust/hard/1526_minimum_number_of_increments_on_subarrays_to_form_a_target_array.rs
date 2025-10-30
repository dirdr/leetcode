impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut steps = target[0];
        for i in 1..target.len() {
            let (curr, prev) = (target[i], target[i - 1]);
            if curr > prev {
                steps += (curr - prev);
            }
        }
        steps
    }
}
