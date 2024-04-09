use std::collections:: {HashMap, HashSet, hash_map::Entry};

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map: HashMap<char, char> = HashMap::new();
        let mut set: HashSet<char> = HashSet::new();
        
        for tup in s.as_str().chars().zip(t.as_str().chars()) {
            let (s_char, t_char) = tup;
            let entry = map.entry(s_char);
            match entry {
                Entry::Vacant(v) => {
                    v.insert(t_char);
                    if set.contains(&t_char) {
                        return false;
                    } else {
                        set.insert(t_char);
                    }
                },
                Entry::Occupied(o) => {
                    if *o.get() != t_char {
                        return false;
                    }
                },
            }
        }
        true
    }
}
