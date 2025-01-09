impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let mut count = 0;
        for word in words {
            if word.starts_with(&pref) {
                count += 1;
            }
        }
        count
    }
}
