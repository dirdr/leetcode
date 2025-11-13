impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let mut s = s.chars().collect::<Vec<_>>();
        let mut ones = 0;
        let mut answer = 0;
        for i in 0..s.len() - 1 {
            if s[i] == '1' {
                ones += 1;
                if s[i + 1] == '0' {
                    answer += ones;
                }
            }
        }
        answer
    }
}
