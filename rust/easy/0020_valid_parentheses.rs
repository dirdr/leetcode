use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut brackets = HashMap::from([(')', '('), (']', '['), ('}', '{')]);
        let mut stack = vec![];
        for ch in s.chars() {
            match brackets.get(&ch) {
                Some(&op) => {
                    if stack.pop() != Some(op) {
                        return false;
                    }
                }
                None => stack.push(ch),
            }
        }
        stack.is_empty()
    }
}
