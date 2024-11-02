impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let mut sentence = sentence.split_whitespace().collect::<Vec<_>>();
        sentence.push(sentence.first().unwrap());
        for i in 1..sentence.len() {
            if sentence[i].chars().nth(0).unwrap() != sentence[i - 1].chars().nth_back(0).unwrap() {
                return false;
            }
        }
        true
    }
}
