use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut count: i32 = 0;
        s.chars().for_each(|letter| {
            map.entry(letter).and_modify(|e| *e += 1).or_insert(1);
            if let Some(val) = map.get(&letter) {
                if val % 2 == 0 {
                    count += 2;
                }
            }
        });
        if s.len() as i32 - count != 0 {count + 1} else {count}
    }
}
