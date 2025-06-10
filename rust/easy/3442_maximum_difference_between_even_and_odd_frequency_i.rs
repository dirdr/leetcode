impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut count = vec![0; 26];
        for ch in s.chars() {
            count[(ch as u8 - b'a') as usize] += 1;
        }
        let (mut min_freq, mut max_freq) = (i32::MAX, 0);
        for &freq in &count {
            if freq == 0 {
                continue;
            }
            if freq % 2 != 0 {
                max_freq = max_freq.max(freq);
            } else {
                min_freq = min_freq.min(freq);
            }
        }
        max_freq - min_freq
    }
}
