use std::collections::VecDeque;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set = VecDeque::new();
        let mut max = 0;
        for c in s.chars() {
            while set.contains(&c) {
                set.pop_front();
            }
            set.push_back(c);
            max = std::cmp::max(set.len(), max);
        }
        max as i32
    }
}
