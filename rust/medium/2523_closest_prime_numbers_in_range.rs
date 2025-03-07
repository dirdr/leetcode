impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let (left, right) = (left as usize, right as usize);
        let mut primes = vec![true; right + 1];
        let mut p = 2;
        while p * p <= right {
            if primes[p] {
                for i in (p * p..right + 1).step_by(p) {
                    primes[i] = false;
                }
            }
            p += 1;
        }
        let mut so_far = -1;
        let mut min_dist = i32::MAX;
        let mut min = vec![-1; 2];
        for i in (left.max(2).max(left)..=right).rev() {
            if !primes[i] {
                continue;
            }
            let i = i as i32;
            if so_far != -1 && (so_far - i) <= min_dist {
                min_dist = (so_far - i);
                min = vec![i, so_far];
            }
            so_far = i;
        }
        min
    }
}
