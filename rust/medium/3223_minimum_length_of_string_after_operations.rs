impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut len = s.len();
        let mut freqs = vec![0; 26];
        for ch in s.chars() {
            freqs[(ch as u8 - b'a') as usize] += 1;
        }
        for i in 0..freqs.len() {
            while freqs[i] >= 3 {
                len -= 2;
                freqs[i] -= 2;
            }
        }
        len as i32
    }
}
