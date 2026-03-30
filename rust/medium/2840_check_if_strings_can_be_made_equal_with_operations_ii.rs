use std::collections::HashMap;

impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        let mut freqs = vec![0; 26 * 2];
        for (i, b) in s1.as_bytes().into_iter().enumerate() {
            let idx = (b - b'a') as usize;
            if i % 2 == 0 {
                freqs[idx] += 1;
            } else {
                freqs[idx * 2] += 1;
            }
        }
        for (i, b) in s2.as_bytes().into_iter().enumerate() {
            let idx = (b - b'a') as usize;
            if i % 2 == 0 {
                freqs[idx] -= 1;
            } else {
                freqs[idx * 2] -= 1;
            }
        }
        for i in 0..26 {
            if freqs[i] != 0 {
                return false;
            }
        }
        true
    }
}
