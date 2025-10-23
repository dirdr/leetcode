use std::collections::VecDeque;

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut current = s.chars().map(|d| d.to_digit(10).unwrap()).collect::<Vec<_>>();
        while current.len() > 2 {
            let mut next = vec![];
            for i in 1..current.len() {
                next.push((current[i] + current[i - 1]) % 10);
            }
            std::mem::swap(&mut current, &mut next);
        }
        current[0] == current[1]
    }
}
