impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        if words.len() == 1 {
            return 0;
        }

        let mut count = 0;
        let mut words = words;
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                let (s1, s2) = (&words[i], &words[j]);
                if s1.len() > s2.len() {
                    continue;
                }
                if Self::is_prefix_and_suffix(s1, s2) {
                    count += 1;
                }
            }
        }
        count
    }

    fn is_prefix_and_suffix(s1: &String, s2: &String) -> bool {
        s2.starts_with(s1) && s2.ends_with(s1)
    }
}
