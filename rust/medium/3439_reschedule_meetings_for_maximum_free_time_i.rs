impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let mut gaps = vec![];
        let mut previous = 0;
        for (&s, &e) in start_time.iter().zip(end_time.iter()) {
            gaps.push(s - previous);
            previous = e;
        }
        gaps.push(event_time - previous);

        let (mut l, mut r) = (0, 0);
        let (mut curr, mut max) = (0, 0);
        while r < gaps.len() {
            curr += gaps[r];
            
            while r - l + 1 > k as usize + 1 {
                curr -= gaps[l];
                l += 1;
            }

            max = max.max(curr);
            r += 1;
        }
        
        max
    }
}
