impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut count = vec![0; 26];
        for c in s.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }
        for c in t.chars() {
            count[(c as u8 - b'a') as usize] -= 1;
        }
        count.iter().all(|&e| e == 0)
    }
}
