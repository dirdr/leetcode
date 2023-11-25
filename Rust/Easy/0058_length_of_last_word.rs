impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.as_str().split_whitespace().rev().next().unwrap().len() as i32
    }
}
