use std::collections::HashSet;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        if s.len() < 10 {return Vec::new()}
        let mut result = HashSet::new();
        let mut set = HashSet::new();
        for i in 0..=s.len() - 10 {
            let slice = String::from(&s[i..i+10]);
            if set.contains(&slice) {
                result.insert(slice);
            } else {
                set.insert(slice);
            }
        }
        result.into_iter().collect::<Vec<String>>()
    }
}
