impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        fn backtrack(s: &String, splits: &mut Vec<String>) -> i32 {
            if s.len() == 0 {
                return splits.len() as i32;
            }
            let mut max = 0;
            for i in 1..=s.len() {
                let split = s[..i].to_owned();
                if !splits.contains(&split) {
                    splits.push(split);
                    let rest = &s[i..s.len()];
                    max = max.max(backtrack(&rest.to_owned(), splits));
                    splits.pop();
                }
            }
            max
        }
        let mut split = vec![];
        backtrack(&s, &mut split)
    }
}
