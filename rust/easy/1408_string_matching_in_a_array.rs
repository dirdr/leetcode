impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut words = words;
        let mut answer = vec![];
        words.sort_unstable_by(|a, b| a.len().cmp(&b.len()));
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if words[j].contains(&words[i]) {
                    answer.push(words[i].clone());
                    break;
                }
            }
        }
        answer
    }
}
