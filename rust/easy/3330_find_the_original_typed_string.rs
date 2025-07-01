impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let word = word.chars().collect::<Vec<_>>();
        word.chunk_by(|a, b| a == b)
            .filter(|c| c.len() >= 2)
            .map(|ch| ch.len() as i32 - 1)
            .sum::<i32>() + 1
    }
}
