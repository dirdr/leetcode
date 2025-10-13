use std::collections::HashMap;

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        fn is_anagram(a: &String, b: &String) -> bool {
            let mut freqs = vec![0; 26];
            for ch in a.chars() {
                freqs[(ch as u8 - b'a') as usize] += 1;
            }
            for ch in b.chars() {
                freqs[(ch as u8 - b'a') as usize] -= 1;
            }
            freqs.iter().all(|&e| e == 0)
        }
        let mut answer = vec![words[0].clone()];
        for i in 1..words.len() {
            if !is_anagram(&words[i], &answer.last().unwrap()) {
                answer.push(words[i].clone());
            } 
        }
        answer
    }
}
