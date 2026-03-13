impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        fn max_worker_crush(w: i64, limit: i64) -> i64 {
            if w > limit {
                return 0;
            }
            let (mut left, mut right) = (1, 1_000_00);
            while left < right {
                let mid = left + (right - left + 1) / 2;
                if w * (mid * (mid + 1) / 2) <= limit {
                    left = mid;
                } else {
                    right = mid - 1;
                }
            }
            left
        }
        let (mut l, mut r) = (1, 1_000_000_000_000_000_000i64);
        while l < r {
            let mut mid = l + (r - l) / 2;
            if worker_times.iter().map(|&w| max_worker_crush(w as i64, mid as i64)).sum::<i64>() >= mountain_height as i64 {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        r as i64
    }
}
