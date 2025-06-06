impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let mut counts = [0; 26];
        for c in s.chars() {
            counts[(c as u8 - b'a') as usize] += 1;
        }
        
        let mut result = String::new();
        let mut t = Vec::new();
        let mut smallest_ch_idx = 0;
        for c in s.chars() {
            t.push(c);
            counts[(c as u8 - b'a') as usize] -= 1;
            
            while smallest_ch_idx < 26 && counts[smallest_ch_idx] == 0 {
                smallest_ch_idx += 1;
            }
            
            let min_remaining = (b'a' + smallest_ch_idx as u8) as char;

            while !t.is_empty() && *t.last().unwrap() <= min_remaining {
                result.push(t.pop().unwrap());
            }
        }
        
        result
    }
}
