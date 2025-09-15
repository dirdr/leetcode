use std::collections::HashSet;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut answer = 0;
        let mut banned = HashSet::<char>::from_iter(broken_letters.chars());
        for word in text.split(' ') {
            if worad.chars().any(|c| banned.contains(&c)) {
                continue
            }
            answer += 1;
        }
        answer
    }
}
