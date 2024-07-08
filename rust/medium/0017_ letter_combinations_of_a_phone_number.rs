use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut answer = vec![];
        if digits.len() == 0 {
            return vec![];
        }
        fn dfs(digits: &[char], letters: &HashMap<usize, Vec<char>>, answer: &mut Vec<String>, curr: usize, current_combination: &mut String) {
            if curr == digits.len() {
                answer.push(current_combination.clone());
                return;
            }
            let current_digit = digits[curr].to_digit(10).unwrap() as usize;
            let floor = letters.get(&current_digit).unwrap();
            for i in 0..floor.len() {
                current_combination.push(floor[i]);
                dfs(digits, letters, answer, curr + 1, current_combination);
                current_combination.pop();
            }
        }
        let letters = HashMap::from([
            (2, vec!['a', 'b', 'c']),
            (3, vec!['d', 'e', 'f']),
            (4, vec!['g', 'h', 'i']),
            (5, vec!['j', 'k', 'l']),
            (6, vec!['m', 'n', 'o']),
            (7, vec!['p', 'q', 'r', 's']),
            (8, vec!['t', 'u', 'v']),
            (9, vec!['w', 'x', 'y', 'z'])
        ]);
        let digits = digits.chars().collect::<Vec<char>>();
        dfs(&digits, &letters, &mut answer, 0, &mut String::new());
        answer
    }
}
