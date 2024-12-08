impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let n = events.len();
        let mut events = events;
        events.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut suffix_max = vec![0; n];
        suffix_max[n - 1] = events[n - 1][2];
        for i in (0..n - 1).rev() {
            suffix_max[i] = suffix_max[i + 1].max(events[i][2]);
        }
        
        let mut max_sum = 0;
        for i in 0..n {
            let next_event = events.partition_point(|e| e[0] <= events[i][1]);
            
            if next_event < n {
                max_sum = max_sum.max(events[i][2] + suffix_max[next_event]);
            }
            max_sum = max_sum.max(events[i][2]);
        }
        max_sum
    }
}
