use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut answer = 0;
        text.split(' ').for_each(|word| {
            let mut set: HashSet<char> = HashSet::from_iter(word.chars().clone());
            let mut flag = true; 
            broken_letters.chars().clone().for_each(|bl| {
                if set.contains(&bl) {flag = false}
            });
            if flag {answer += 1}
        });
        answer
    }
}
