use std::collections::HashSet;

impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']);
        let mut freqs = vec![0; 26];
        for ch in s.chars() {
            freqs[(ch as u8 - b'a') as usize] += 1;
        }
        let (mut max, mut max_vowel) = (0, 0);
        for point in 0..26 {
            if vowels.contains(&((point as u8 + b'a') as char)) {
                max_vowel = max_vowel.max(freqs[point]);
            } else {
                max = max.max(freqs[point]);
            }
        }
        max  as i32 + max_vowel as i32
    }
}
