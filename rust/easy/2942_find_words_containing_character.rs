impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        words.iter().enumerate().fold(vec![], |mut acc, (i, word)| {
            if let Some(_) = word.chars().find(|ch| ch == &x) {
                acc.push(i as i32);
            }
            acc
        })
    }
}
