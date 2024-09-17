use std::collections::HashMap;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut map: HashMap<String, usize> = HashMap::new();
        let mut s = s1;
        s.push((' '));
        s.push_str(&s2);
        for token in s.split(' ') {
            map.entry(token.to_string()).and_modify(|e| *e += 1).or_insert(1);
        }
        map.into_iter().filter(|(_, v)| *v == 1).map(|(k, _)| k).collect::<Vec<String>>()
    }
}
