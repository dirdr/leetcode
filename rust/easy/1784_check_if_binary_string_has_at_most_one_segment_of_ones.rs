impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let s = s.chars().collect::<Vec<char>>();
        for i in 1..s.len() {
            if s[i - 1] == '0' && s[i] == '1' {
                return false;
            }
        }
        true
    }
}
