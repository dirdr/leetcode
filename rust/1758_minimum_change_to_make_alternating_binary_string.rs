impl Solution {
    pub fn min_operations(mut s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        fn count(start: char, s: &[char]) -> i32 {
            let mut ops = 0;
            let mut prev = start;
            if start != s[0] {
                ops += 1;
            }
            for i in 1..s.len() {
                if s[i] == prev {
                    ops += 1;
                    if s[i] == '1' {
                        prev = '0';
                    } else {
                        prev = '1';
                    }
                } else {
                    prev = s[i];
                }
            }
            ops
        }
        count('1', &s).min(count('0', &s))
    }
}
