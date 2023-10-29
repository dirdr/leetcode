use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut map = HashMap::new();
        s.chars().for_each(|l| {
            map.entry(l)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        });
        let mut vectorized = map.iter().collect::<Vec<(&char, &i32)>>();
        vectorized.sort_unstable_by(|a, b| b.1.cmp(a.1));
        let mut result = Vec::new();
        vectorized.into_iter().for_each(|(key, value)| {
            for _ in 0..*value {
                result.push(key.clone());
            }
        });
        result.iter().collect::<String>()
    }
}
