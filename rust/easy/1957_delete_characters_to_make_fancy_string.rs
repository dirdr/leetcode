impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut stack = Vec::with_capacity(s.len());
        let s = s.as_bytes();
        stack.push(s[0]);
        let mut consecutive = 1;
        for &ch in s.iter().skip(1) {
            let &l = stack.last().unwrap();
            if ch == l {
                if consecutive < 2 {
                    stack.push(ch);
                    consecutive += 1;
                }
            } else {
                stack.push(ch);
                consecutive = 1;
            }
        }
        stack.into_iter().map(|bc| bc as char).collect::<String>()
    }
}
