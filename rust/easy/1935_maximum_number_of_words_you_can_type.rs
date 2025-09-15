use std::collections::HashSet;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut banned = HashSet::<char>::from_iter(broken_letters.chars());
        text.split(' ')
            .filter(|w| w.chars().all(|c| !banned.contains(&c)))
            .count() as i32
    }
}
