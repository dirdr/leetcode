impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        fn backtrack(n: usize, k: usize, current: &mut Vec<char>, out: &mut Vec<String>) {
            if out.len() >= k {
                return;
            }
            if current.len() == n {
                out.push(current.iter().cloned().collect::<String>());
                return;
            }
            for &ch in &['a', 'b', 'c'] {
                if let Some(&last) = current.last() {
                    if last == ch {
                        continue;
                    }
                }
                current.push(ch);
                backtrack(n, k, current, out);
                current.pop();
            }
        }
        let mut out = vec![];
        backtrack(n as usize, k as usize, &mut vec![], &mut out);
        if k as usize > out.len() {
            return String::from("");
        }
        out.last().unwrap().to_owned()
    }
}
