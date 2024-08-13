use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            if let Some(v) = map.get_mut(&c) {
                *v = -1;
            } else {
                map.insert(c, i as i32);
            }
        }
        match map.iter().filter(|(k, &v)| v !f= -1).map(|(k, v)| v).min() {
            Some(&v) => v,
            None => -1
        }
    }
}
