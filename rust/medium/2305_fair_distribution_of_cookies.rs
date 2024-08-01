impl Solution {
    pub fn distribute_cookies(cookies: Vec<i32>, k: i32) -> i32 {
        fn backtrack(cookies: &[i32], k: usize, dist: &mut Vec<i32>, idx: usize, curr: i32) -> i32 {
            if idx == cookies.len() {
                return *dist.iter().max().unwrap();
            }
            let mut min = curr;
            for i in 0..k {
                dist[i] += cookies[idx];
                if dist[i] < min {
                    min = min.min(backtrack(cookies, k, dist, idx + 1, min));
                }
                dist[i] -= cookies[idx];
                if dist[i] == 0 {
                    break;
                }
            }
            min
        }
        let mut cookies = cookies.clone();
        cookies.sort_by(|a, b| b.cmp(a));
        backtrack(&cookies, k as usize, &mut vec![0; k as usize], 0, i32::MAX)
    }
}
