use std::collections::HashSet;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        if s.len() < k as usize {
            return false;
        }
        HashSet::<&str>::from_iter((0..s.len() - k as usize + 1).map(|i| &s[i..i + k as usize])).len() == 2usize.pow(k as u32)
    }
}
