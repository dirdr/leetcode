use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map: HashMap<char, char> = HashMap::new();
        let mut card_t: HashSet<char> = HashSet::new();
        let mut card_s: HashSet<char> = HashSet::new();
        
        for tup in s.as_str().chars().zip(t.as_str().chars()) {
            let (s_char, t_char) = tup;
            card_t.insert(s_char);
            card_s.insert(t_char);
            let entry = map.entry(s_char);
            match entry {
                Entry::Vacant(v) => {
                    v.insert(t_char);
                },
                Entry::Occupied(o) => {
                    if *o.get() != t_char {
                        return false;
                    }
                },
            }
        }
        card_t.len() == card_s.len()
    }
}
