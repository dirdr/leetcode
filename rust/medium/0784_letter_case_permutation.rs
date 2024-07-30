impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        fn backtrack(s: &[char], current: &mut Vec<char>, result: &mut Vec<String>, pos: usize) {
            if pos == s.len() {
                result.push(current.iter().collect());
                return;
            }
            let ch = s[pos];
            current.push(ch);
            backtrack(s, current, result, pos + 1);
            current.pop();
            if ch.is_alphabetic() {
                let opposite = if ch.is_ascii_lowercase() { ch.to_ascii_uppercase() } else { ch.to_ascii_lowercase() };
                current.push(opposite);
                backtrack(s, current, result, pos + 1);
                current.pop();
            }
        }
        let mut result = Vec::new();
        let chars: Vec<char> = s.chars().collect();
        backtrack(&chars, &mut Vec::new(), &mut result, 0);
        result
    }
}
