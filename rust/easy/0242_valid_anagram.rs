use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut ms = HashMap::new();
        let mut mt = HashMap::new();
        for ch in s.chars() {
            ms.entry(ch).and_modify(|e| *e += 1).or_insert(1);
        }
        for ch in t.chars() {
            mt.entry(ch).and_modify(|e| *e += 1).or_insert(1);
        } 
        ms == mt
    }
}
