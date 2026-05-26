impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let (mut low, mut upp) = (vec![0; 26], vec![0; 26]);
        for ch in word.chars() {
            if ch.is_ascii_uppercase() {
                upp[(ch as u8 - b'A') as usize] += 1;
            }
            if ch.is_ascii_lowercase() {
                low[(ch as u8 - b'a') as usize] += 1;
            }
        }
        (0..26).filter(|&i| low[i] >= 1 && upp[i] >= 1).count() as i32
    }
}
