impl Solution {
    pub fn max_score(s: String) -> i32 {
        let (mut max, mut score) = (0, 0);
        let s = s.chars().collect::<Vec<_>>();
        for &c in &s {
            if c == '1' {
                score += 1;
            }
        }
        for &c in s.iter().take(s.len() - 1) {
            if c == '1' {
                score -= 1;
            } else {
                score += 1;
            }
            max = max.max(score);
        }
        max
    }
}
