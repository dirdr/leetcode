use std::collections::HashSet;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prefix = Vec::with_capacity(words.len());
        let mut sum = 0;
        let mut vowels = HashSet::<char>::from_iter(vec!['a', 'e', 'i', 'o', 'u'].into_iter());
        for word in &words {
            if vowels.contains(&word.chars().next().unwrap())
                && vowels.contains(&word.chars().nth(word.len() - 1).unwrap())
            {
                sum += 1;
            }
            prefix.push(sum);
        }
        let mut answer = Vec::with_capacity(queries.len());
        for q in queries {
            let mut count = prefix[q[1] as usize];
            if q[0] != 0 {
                count -= prefix[q[0] as usize - 1];
            }
            answer.push(count);
        }
        answer
    }
}
