use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut helper = HashMap::from([('{', '}'), ('[', ']'), ('(', ')')]);
        let mut stack: Vec<char> = vec![];
        for ch in s.chars() {
            if helper.contains_key(&ch) {
                stack.push(helper.get(&ch).unwrap().to_owned());
            } else {
                if stack.is_empty() {
                    return false;
                }
                if ch != stack.pop().unwrap() {
                    return false;
                }
            }
        }
        stack.is_empty()
    }
}
