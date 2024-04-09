use std::collections::HashMap;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut map = HashMap::new();
        for word in words1 {
            map.entry(word)
                .and_modify(|e| *e = -1)
                .or_insert(1);
        }
        for word in words2 {
            map.entry(word)
                .and_modify(|e| *e -= 1);
        }
        map.iter()
            .filter(|el| *el.1 == 0)
            .count() as i32
    }
}
