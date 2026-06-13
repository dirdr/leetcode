impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        fn weight(word: &[char], weights: &[i32]) -> i32 {
            let mut sum = 0;
            for &ch in word {
                sum += weights[(ch as u8 - b'a') as usize];
            }
            sum
        }
        words.into_iter().map(|w| {
            let w = weight(&w.chars().collect::<Vec<_>>(), &weights) % 26;
            (25 - w as u8 + b'a') as char
        }).collect::<String>()
    }
}
