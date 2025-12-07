impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let mut between = high - low + 1;
        if low % 2 == 0 && high % 2 == 0 {
            between -= 1;
        } else if low % 2 != 0 && high % 2 != 0 {
            between += 1;
        }
        between / 2
    }
}
