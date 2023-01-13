use std::collections::HashMap;

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut map = HashMap::new();
        for (i, el) in order.chars().enumerate() {
            map.entry(el).or_insert(i);
        }
        // partition will return two collections, one with element when closure return true,
        // one with closure returning false
        let (mut part, other): (Vec<char>, Vec<char>) = s.clone().chars().partition(|el| map.contains_key(el));
        part.sort_by(|a, b| {   
            map.get(&a).unwrap().cmp(map.get(&b).unwrap())  
        });
        part.into_iter()
            .chain(other)
            .collect::<String>()
    }
}
