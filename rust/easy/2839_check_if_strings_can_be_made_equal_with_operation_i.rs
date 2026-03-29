impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let s1 = s1.chars().collect::<Vec<_>>();
        let s2 = s2.chars().collect::<Vec<_>>();
        (s1[0] == s2[0] || s1[0] == s2[2]) &&
        (s1[2] == s2[0] || s1[2] == s2[2]) &&
        (s1[1] == s2[1] || s1[1] == s2[3]) &&
        (s1[3] == s2[1] || s1[3] == s2[3])
    }
}
