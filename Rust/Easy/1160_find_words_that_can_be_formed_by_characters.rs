use std::collections::HashMap;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut map = Solution::get_freq_map(&chars);
        let mut count = 0;
        for word in words.iter() {
            let mut flag = true;
            let mut temp = Solution::get_freq_map(&word);
            temp.iter().for_each(|(key, value)| {
                if !map.contains_key(key) || map.get(key).unwrap() < value {flag = false}
            });
            if flag == true {count += word.len()}
        }
        count as i32
    }

    pub fn get_freq_map(word: &String) -> HashMap<char, i32> {
        let mut map = HashMap::new();
        word.chars().for_each(|ch| {
            map.entry(ch).and_modify(|e| *e += 1).or_insert(1);
        });
        map
    }
}
