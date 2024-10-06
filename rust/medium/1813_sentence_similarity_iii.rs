impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let words1: Vec<&str> = sentence1.split_whitespace().collect();
        let words2: Vec<&str> = sentence2.split_whitespace().collect();
        let (len1, len2) = (words1.len(), words2.len());
        let mut i = 0;
        while i < len1 && i < len2 && words1[i] == words2[i] {
            i += 1;
        }
        let mut j = 0;
        while j < len1 - i && j < len2 - i && words1[len1 - 1 - j] == words2[len2 - 1 - j] {
            j += 1;
        }
        i + j == len1.min(len2)
    }
}
