impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns.into_iter().filter(|p| word.contains(p)).count() as i32
    }
}
