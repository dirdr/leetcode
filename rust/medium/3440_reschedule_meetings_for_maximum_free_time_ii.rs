use std::collections::HashSet;

impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let mut spaces = vec![];
        let mut prev = 0;
        for (&s, &e) in start_time.iter().zip(end_time.iter()) {
            spaces.push(s - prev);
            spaces.push(e - s);
            prev = e;
        }
        spaces.push(event_time - prev);

        let mut max = 0;
        let mut biggest = 0;
        for w in spaces.windows(3).step_by(2) {
            max = max.max(w[0] + w[2]);
            if biggest >= w[1] {
                max = max.max(w[0] + w[1] + w[2]);
            }
            biggest = biggest.max(w[0]);
        }

        biggest = 0;
        for w in spaces.windows(3).rev().step_by(2) {
            if biggest >= w[1] {
                max = max.max(w[0] + w[1] + w[2]);
            }
            biggest = biggest.max(w[2]);
        }
        max
    }
}
