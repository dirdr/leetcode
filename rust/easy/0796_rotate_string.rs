impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        let goal = goal.as_bytes();
        let mut s = s.as_bytes().to_vec();
        for _ in 0..s.len() {
            s.rotate_right(1);
            if s == goal {
                return true;
            }
        }
        false
    }
}
