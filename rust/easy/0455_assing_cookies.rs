impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g;
        let mut s = s;
        g.sort_unstable_by(|a, b| b.cmp(&a));
        s.sort_unstable_by(|a, b| b.cmp(&a));
        let mut answer = 0;
        let (mut i, mut j) = (0, 0);
        while i < g.len() && j < s.len() {
            if s[j] >= g[i] {
                answer += 1;
                j += 1;
            }
            i += 1;
        }
        answer
    }
}
