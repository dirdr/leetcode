impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let (mut l, mut r) = (0, s.len() - 1);
        while l < r {
            (s[l], s[r]) = (s[r], s[l]);
            l += 1;
            r -= 1;
        }
    }
}
