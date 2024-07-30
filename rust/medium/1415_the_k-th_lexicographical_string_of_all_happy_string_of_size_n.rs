impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        fn backtrack(n: i32, current: &mut String, all: &mut Vec<String>) {
            if current.len() == n as usize {
                all.push(current.clone());
                return;
            }
            for l in ['a', 'b', 'c'] {
                if let Some(last) = current.chars().last() {
                    if last == l {
                        continue;
                    }
                }
                current.push(l);
                backtrack(n, current, all);
                current.pop();
            }
        }
        let mut result = vec![];
        backtrack(n, &mut String::new(), &mut result);
        result.sort_unstable();
        if let Some(kth) = result.iter().nth(k as usize - 1) {
            return kth.to_owned();
        }
        "".to_owned()
    }
}
