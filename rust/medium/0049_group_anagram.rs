use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut ans: HashMap<Vec<usize>, Vec<String>> = HashMap::new();
        for el in strs.iter() {
            let count = el.clone().chars().fold(vec![0; 26], |mut vec, ch| {
                vec[ch as usize - 'a' as usize] += 1;
                vec
            });
            ans.entry(count)
                .and_modify(|e| e.push(el.clone()))
                .or_insert(vec![el.clone()]);
        }
        ans.iter()
            .map(|(k, v)| v.clone())
            .collect::<Vec<Vec<String>>>()
    }
}
