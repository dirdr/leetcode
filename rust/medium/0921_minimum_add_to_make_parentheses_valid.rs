impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut cnt = 0;
        let mut unmatched = 0;
        for ch in s.chars() {
            if ch == '(' {
                unmatched += 1;
            } else {
                if unmatched > 0 {
                    unmatched -= 1
                } else {
                    cnt += 1
                }
            }
        }
        cnt + unmatched
    }
}
