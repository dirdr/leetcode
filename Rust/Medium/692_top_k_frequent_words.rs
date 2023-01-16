use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, mut k: i32) -> Vec<String> {
        let mut map: HashMap<String, i32> = HashMap::new();
        words.iter().for_each(|w| {
            map.entry(w.clone()).and_modify(|e| *e += 1).or_insert(1);
        });
        let mut answer = map.into_iter().collect::<Vec<(String, i32)>>();
        answer.sort_unstable_by(|(k1, v1), (k2, v2)| v2.cmp(v1).then(k1.cmp(k2)));
        answer.into_iter()
            .map(|(key, _)| key)
            .take(k as usize)
            .collect::<Vec<String>>()
    }
}
