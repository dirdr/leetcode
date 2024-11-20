impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let mut freqs = vec![0; 3];
        for &r in s.as_bytes().iter().rev() {
            freqs[(r - b'a') as usize] += 1;
        }
        if freqs.iter().any(|&f| f < k) {
            return -1;
        }
        let (mut l, mut r) = (-1, 0);
        let mut bytes = s.as_bytes().iter().collect::<Vec<_>>();
        let mut min = i32::MAX;
        while r < bytes.len() {
            let current_right = (bytes[r] - b'a') as usize;
            freqs[current_right] -= 1;
            r += 1;
            while freqs[current_right] < k {
                l += 1;
                freqs[(bytes[l as usize] - b'a') as usize] += 1;
            }
            min = min.min(l + 1 + bytes.len() as i32 - r as i32);
        }
        min
    }
}
