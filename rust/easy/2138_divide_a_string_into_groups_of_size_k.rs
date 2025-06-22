impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        let s = s.chars().collect::<Vec<_>>();
        let mut answer = s.chunks(k as usize)
            .map(|s| s.to_vec())
            .collect::<Vec<Vec<char>>>();

        let mut last = answer.last_mut().unwrap();
        while last.len() < k as usize {
            last.push(fill);
        }
        
        answer.into_iter()
            .map(|v| v.iter().collect::<String>())
            .collect::<Vec<_>>()
    }
}
