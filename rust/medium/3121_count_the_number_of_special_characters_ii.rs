impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut first = vec![-1; 26];
        let mut last = vec![-1; 26];
        for (i, ch) in word.chars().enumerate() {
            if ch.is_ascii_uppercase() {
                let idx = (ch as u8 - b'A') as usize;
                if first[idx] == -1 {
                    first[idx] = i as i32;
                } 
            } else {
                last[(ch as u8 - b'a') as usize] = i as i32;
            }
        }
        let mut count = 0;
        for i in 0..26 {
            if first[i] != -1 && last[i] != -1 {
                if first[i] > last[i] {
                    count += 1;
                }
            }
        }
        count
    }
}
