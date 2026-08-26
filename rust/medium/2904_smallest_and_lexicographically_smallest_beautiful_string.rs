impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        fn is_beautiful(s: &str, k: i32) -> bool {
            s.bytes().filter(|&b| b == b'1').count() == k as usize
        }

        let mut smallest: &str = "";
        for i in 0..s.len() {
            for j in i..s.len() {
                let candidate = &s[i..=j];
                if is_beautiful(candidate, k)
                    && (smallest.is_empty()
                        || candidate.len() < smallest.len()
                        || (candidate.len() == smallest.len() && candidate < smallest))
                {
                    smallest = candidate;
                }
            }
        }
        smallest.to_string()
    }
}
