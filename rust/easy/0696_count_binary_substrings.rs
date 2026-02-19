impl Solution {
    pub fn count_binary_substrings(mut s: String) -> i32 {
        let mut answer = 0;
        let chunks = s.as_bytes().chunk_by(|a, b| a == b);
        let mut prev = -1;
        for chunk in chunks {
            if prev == -1 {
                prev = chunk.len() as i32;
                continue;
            }
            let curr = chunk.len() as i32;
            answer += prev.min(curr);
            prev = curr;
        }
        answer
    }
}
