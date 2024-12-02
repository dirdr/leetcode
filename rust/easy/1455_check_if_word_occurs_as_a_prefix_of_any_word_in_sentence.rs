impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let mut count = 0;
        for word in sentence.split_whitespace() {
            count += 1;
            if word.starts_with(&search_word) {
                return count;
            }
        }
        -1
    }
}
