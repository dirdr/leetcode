use std::collections::HashSet;

impl Solution {
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let banned: HashSet<i32> = HashSet::from_iter(banned.into_iter());
        let mut sum = 0;
        (1..=n).filter(|n| !banned.contains(n)).take_while(|n| {
          w  sum += n;
            sum <= max_sum
        }).count() as i32
    }
}
