impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        words
            .iter()
            .filter(|&word| word.chars().all(|c| allowed.contains(c)))
            .count() as i32
    }
}
