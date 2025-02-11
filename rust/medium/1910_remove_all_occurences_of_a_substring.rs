impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut s = s;
        let mut part_len = part.len();
        while let Some(idx) = s.find(&part) {
            for _ in idx..idx + part_len {
                s.remove(idx);
            }
        }
        s
    }
}
