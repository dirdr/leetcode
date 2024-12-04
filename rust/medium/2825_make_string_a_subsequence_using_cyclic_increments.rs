impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let (mut p1, mut p2) = (0, 0);
        let (str1, str2) = (str1.chars().collect::<Vec<_>>(), str2.chars().collect::<Vec<_>>());
        while p2 < str2.len() && p1 < str1.len() {
            if str1[p1] == str2[p2] || (((str1[p1] as u8 - b'a' + 1) % 26) + b'a') as char == str2[p2] {
                p2 += 1;
            }
            p1 += 1;
        }
        p2 == str2.len()
    }
}
