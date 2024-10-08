impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let opening = s.chars()
            .fold(0usize, |acc, ch| {
                match ch {
                    '[' => acc + 1,
                    ']' => acc.saturating_sub(1),
                    _ => unreachable!(),
                }
            });
        
        (opening as i32 + 1) / 2
    }
}
