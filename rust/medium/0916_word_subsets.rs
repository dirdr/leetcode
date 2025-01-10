impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut requirements = vec![0; 26];
        let mut answer = Vec::with_capacity(words1.len());
        for word in words2 {
            let mut freq = vec![0; 26];
            for ch in word.chars() {
                freq[(ch as u8 - b'a') as usize] += 1;
            }
            for i in 0..26 {
                requirements[i] = requirements[i].max(freq[i]);
            }
        }
        for word in words1 {
            let mut temp = requirements.clone();
            for ch in word.chars() {
                let entry = temp.get_mut((ch as u8 - b'a') as usize).unwrap();
                *entry -= 1;
            }
            if !temp.iter().any(|&f| f > 0) {
                answer.push(word.clone());
            }
        }
        answer
    }
}
