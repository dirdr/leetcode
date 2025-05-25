use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut count = HashMap::new();
        for word in words {
            *count.entry(word).or_insert(0) += 1;
        }

        let mut answer = 0;
        let mut center_used = false;
        for (word, freq) in count.iter() {
            let rev: String = word.chars().rev().collect();
            if word == &rev {
                answer += (freq / 2) * 4;
                if freq % 2 == 1 && !center_used {
                    answer += 2;
                    center_used = true;
                }
            } else if word < &rev {
                if let Some(&rev_freq) = count.get(&rev) {
                    answer += freq.min(&rev_freq) * 4;
                }
            }
        } 
        answer
    }
}
