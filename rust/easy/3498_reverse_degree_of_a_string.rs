impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        let mut ans = 0;
        for (i, v) in s.as_bytes().iter().enumerate() {
            ans += (26 - (v - b'a')) as i32 * (i as i32 + 1);
        }
        ans
    }
}
