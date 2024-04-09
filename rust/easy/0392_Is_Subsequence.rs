impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() == 0 {
            return true;
        }
        let mut s_index = 0;
        let mut s_vec = s.chars().collect::<Vec<char>>();
        
        for ch in t.as_str().chars() {
            if s_index == s.len() {
                return true;
            }
            if s_vec[s_index] == ch {
                s_index += 1;
            }
        }
        s_index == s.len()
    }
}
