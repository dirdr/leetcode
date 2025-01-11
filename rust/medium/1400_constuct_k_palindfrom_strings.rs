impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        if s.len() < k as usize {
            return false;
        }
        if s.len() == k as usize {
            return true;
        }
        let mut freqs = vec![0; 26];
        for ch in s.chars() {
            freqs[(ch as u8 - b'a') as usize] += 1;
        }
        let mut odd_count = 0;
        for fr in &freqs {
            if fr % 2 != 0 {
                odd_count += 1;
            }
        }
        if odd_count > k {
            return false;
        }
        return true;
    }
}
