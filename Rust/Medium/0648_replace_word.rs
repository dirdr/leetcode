use std::collections::HashMap;

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut map: HashMap<String, String> = HashMap::new();
        for word in sentence.split(" ") {
            for prefix in dictionary.iter().cloned() {
                if (word.starts_with(&prefix)) {
                    map.entry(word.to_string()).and_modify(|e| {
                        if prefix.len() < e.len() {*e = prefix.clone();} 
                    }).or_insert(prefix.clone());
                }
            }
        }
        sentence.split(" ").map(|word| {
            if let Some(prefix) = map.get(&word.to_string()) {prefix.clone()} else {String::from(word)}
        }).collect::<Vec<String>>().join(" ")
    }
}
