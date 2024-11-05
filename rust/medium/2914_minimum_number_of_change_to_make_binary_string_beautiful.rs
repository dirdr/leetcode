impl Solution {
    pub fn min_changes(s: String) -> i32 {
        s.as_bytes()
            .chunks(2)
            .map(|c| if c[0] == c[1] { 0 } else { 1 })
            .sum::<i32>()
    }
}
