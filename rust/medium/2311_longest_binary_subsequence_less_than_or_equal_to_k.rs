impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let (mut pows, mut sum, mut len) = (vec![], 0, s.len() as i32);
        for (idx, ch) in s.chars().enumerate() {
            if ch == '1' {
                let pow = (s.len() - idx - 1) as u32;
                if let Some(val) = 2i32.checked_pow(pow) {
                    pows.push(pow);
                    sum += val;
                } else {
                    len -= 1;
                }
            }
        }
        if sum <= k as i32 {
            return len;
        }
        for i in 0..pows.len() {
            sum -= 2_i32.pow(pows[i]);
            len -= 1;
            if sum <= k as i32 {
                break;
            }
        }
        len
    }
}
